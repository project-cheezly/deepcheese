import logging
import random
from multiprocessing import Pool

import polars as pl
import numpy as np
import torch

from deepcheese.data_loader import DataLoader
from static_conf import Config


logger = logging.Logger(__name__)


class MovingWindow(torch.utils.data.Dataset):
    """
    Basic Moving window dataset for limit order book data
    """
    def __init__(self, data: tuple[str, pl.DataFrame]):
        """
        Initialize moving window dataset
        :param data: tuple of file name and limit order book data
        """
        self._config = Config().dataset.moving_window

        file_name, df = data

        # get the entire matrix min and max
        self._local_min = df.select(pl.selectors.matches("*p").min()).to_numpy().min() - self._config.price_window
        self._local_max = df.select(pl.selectors.matches("*p").max()).to_numpy().max() + self._config.price_window

        # get the best bid and ask price
        self._best_ask = df.select(pl.selectors.matches("a1p")).to_numpy()
        self._best_bid = df.select(pl.selectors.matches("b1p")).to_numpy()
        self._mid_price = np.round((self._best_ask + self._best_bid) / 2).astype(np.int32) - self._local_min

        # get the entire limit order book array
        self.arr = self._get_array(df)

        # get the start, last index
        self._start_idx = self._get_start_idx()
        self._end_idx = self._get_end_idx()

        # get the label
        self.label = self._fill_label()

    def __len__(self):
        """
        Get the length of the dataset
        """
        return self._end_idx - self._start_idx + 1

    def __getitem__(self, item):
        """
        Get item from the dataset

        Adjust the item index to the actual index of the limit order book array
        to prevent index out of range error and adjust offset
        """
        idx = item + self._start_idx
        mid_price = int(self._mid_price[idx])

        return np.expand_dims(self.arr[
            idx - self._config.time_window + 1: idx + 1,
            mid_price - self._config.price_window: mid_price + self._config.price_window + 1
        ], axis=0), self.label[item]

    def _get_start_idx(self) -> int:
        return max(self._config.time_window - 1, self._config.predict_horizon)

    def _get_end_idx(self) -> int:
        return self.arr.shape[0] - self._config.predict_horizon

    def _get_array(self, df: pl.DataFrame) -> np.ndarray:
        """
        Get entire limit order book array from cache or generate it
        :param df: limit order book data
        :return: entire limit order book array
        """
        return self._transform_df_to_array(df)

    def _transform_df_to_array(self, df: pl.DataFrame) -> np.ndarray:
        """
        transform limit order book data to entire limit order book array

        Cached for performance. Delete the cache if the data changes.

        :param df: singular limit order book data
        :return: entire limit order book array
        """
        _bid_keys = [(f"b{key}p", f"b{key}a") for key in range(1, 6)]
        _ask_keys = [(f"a{key}p", f"a{key}a") for key in range(1, 6)]

        entire_matrix = np.zeros((len(df), self._local_max - self._local_min + 1)).astype(np.int32)

        # fill the matrix with the limit order book data
        for idx, row in enumerate(df.rows(named=True)):
            for key in _bid_keys:
                # fill the matrix with the bid data (positive)
                entire_matrix[idx][df[idx, key[0]] - self._local_min:] += df[idx, key[1]]
            for key in _ask_keys:
                # fill the matrix with the ask data (negative)
                entire_matrix[idx][:df[idx, key[0]] - self._local_min + 1] -= df[idx, key[1]]

        entire_matrix = entire_matrix.clip(min=-40, max=40).astype(np.float32)
        entire_matrix *= 1/40
        return entire_matrix

    def _fill_label(self):
        label = np.zeros(self._end_idx - self._start_idx + 1, dtype=np.int32)

        for idx in range(self._start_idx, self._end_idx + 1):
            m_prev = np.mean(self._best_ask[idx - self._config.predict_horizon:idx])
            m_next = np.mean(self._best_bid[idx + 1:idx + self._config.predict_horizon + 1])
            l = (-m_next + m_prev) / m_prev

            if l < -0.001:
                label[idx - self._start_idx] = 1
            else:
                label[idx - self._start_idx] = 0

        return label


class MultipleMovingWindow(MovingWindow):
    """
    Moving window dataset for limit order book data having multiple window sizes.

    Three different window sizes are considered for the dataset.
    """
    def __init__(self, data: tuple[str, pl.DataFrame]):
        """
        Initialize moving window dataset
        :param data: tuple of file name and limit order book data
        """
        super().__init__(data)

        self._config = Config().dataset.multiple_moving_window

        self._start_idx = max(self._config.large.time_window - 1, self._config.predict_horizon)
        self._end_idx = self.arr.shape[0] - self._config.predict_horizon

        self._label = self._fill_label()

    def __getitem__(self, item):
        idx = item + self._start_idx
        mid_price = int(self._mid_price[idx])

        return (np.expand_dims(self.arr[
                idx - self._config.large.time_window + 1: idx + 1,
                mid_price - self._config.large.price_window: mid_price + self._config.large.price_window + 1
            ], axis=0), np.expand_dims(self.arr[
                idx - self._config.medium.time_window + 1: idx + 1,
                mid_price - self._config.medium.price_window: mid_price + self._config.medium.price_window + 1
            ], axis=0), np.expand_dims(self.arr[
                idx - self._config.small.time_window + 1: idx + 1,
                mid_price - self._config.small.price_window: mid_price + self._config.small.price_window + 1
            ], axis=0)), self.label[idx]


class MovingWindow3D(MovingWindow):
    """
    Moving window dataset for limit order book data having 3D window.
    """
    def __init__(self, data: tuple[str, pl.DataFrame]):
        """
        Initialize moving window dataset
        :param data: tuple of file name and limit order book data
        """
        self._concat_size = Config().dataset.moving_window_3d.concat_image_size
        super().__init__(data)
        print(np.bincount(self.label))

    #     self.filtered_idx = []
    #     for idx in range(self._end_idx - self._start_idx + 1):
    #         if self.label[idx] == 1:
    #             self.filtered_idx.append(idx)
    #         else:
    #             if random.random() < 0.01:
    #                 self.filtered_idx.append(idx)
    #
    # def __len__(self):
    #     return len(self.filtered_idx)

    def _get_start_idx(self) -> int:
        return max(self._config.time_window + self._concat_size * 10 - 1, self._config.predict_horizon + 1)

    def _get_end_idx(self) -> int:
        return self.arr.shape[0] - self._config.predict_horizon

    def __getitem__(self, item):
        idx = item + self._start_idx

        result = []
        for i in range(self._concat_size):
            offset = i * 10
            mid_price = int(self._mid_price[idx - offset])
            result.append(np.expand_dims(self.arr[
                idx - offset - self._config.time_window + 1: idx - offset + 1,
                mid_price - self._config.price_window: mid_price + self._config.price_window + 1
            ], axis=0))

        return np.expand_dims(np.concatenate(result, axis=0), axis=0), self.label[idx - self._start_idx]


class MovingWindowDataset:
    """
    Moving window dataset for limit order book data
    """
    def __init__(self, data_loader: DataLoader):
        """
        Initialize moving window dataset
        :param data_loader: limit order book data loader
        """
        self.test_data = None
        self.train_data = None
        self._config = Config().dataset.moving_window

        self.windows = self._generate_windows(data_loader)

    def _generate_windows(self, data_loader: DataLoader) -> list[MovingWindow]:
        """
        Generate moving window dataset
        :param data_loader: limit order book data loader
        """
        with Pool() as pool:
            return pool.map(MovingWindow, data_loader.lob_data())

    def get_test_day(self, idx):
        return self.test_data[idx]

    def get_data_loader(self) -> tuple[
            list[torch.utils.data.DataLoader],
            list[torch.utils.data.DataLoader]
    ]:
        """
        Get the data loader for the dataset
        :return: train, validation, test data loader
        """
        val_ratio = Config().train.val_ratio
        test_ratio = Config().train.test_ratio

        days = len(self)
        test_days = int(days * test_ratio)
        train_days = days - test_days

        self.train_data = self.windows[:train_days]
        self.test_data = self.windows[train_days:]

        return (self._generate_data_loader(self.train_data),
                self._generate_data_loader(self.test_data))

    def _generate_data_loader(self, data: list[MovingWindow]) -> list[torch.utils.data.DataLoader]:
        """
        Generate data loader for the dataset
        :param data: list of moving window dataset
        :return: list of data loader
        """
        return [torch.utils.data.DataLoader(window, batch_size=64, num_workers=4) for window in data]

    def __getitem__(self, idx) -> MovingWindow:
        return self.windows[idx]

    def __len__(self):
        return len(self.windows)


class MultipleMovingWindowDataset(MovingWindowDataset):
    """
    Multiple moving window dataset for limit order book data
    """

    def _generate_windows(self, data_loader: DataLoader):
        """
        Generate multiple moving window dataset
        :param data_loader: limit order book data loader
        """
        with Pool() as pool:
            return pool.map(MultipleMovingWindow, data_loader.lob_data())


class MovingWindow3dDataset(MovingWindowDataset):
    """
    Moving window 3D dataset for limit order book data
    """

    def _generate_windows(self, data_loader: DataLoader) -> list[MovingWindow]:
        """
        Generate moving window 3D dataset
        :param data_loader: limit order book data loader
        """
        with Pool() as pool:
            return pool.map(MovingWindow3D, data_loader.lob_data())
