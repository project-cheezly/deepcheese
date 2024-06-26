import pathlib
from typing import Iterator

import polars as pl

from deepcheese.data_loader import DataLoader
from static_conf import Config


class IndiLoader(DataLoader):
    """
    Indi dataset loader
    """
    def __init__(self):
        self._config = Config().data_loader

    def lob_data(self) -> Iterator[tuple[str, pl.DataFrame]]:
        """
        Load limit order book data
        :return: a tuple of file name and limit order book data iterator
        """
        return self._load_limit_order_book()

    def price_data(self) -> Iterator[pl.DataFrame]:
        """
        Load price data
        :return: price data iterator
        """
        return self._load_price()

    def _load_limit_order_book(self) -> Iterator[tuple[str, pl.DataFrame]]:
        """
        Load limit order book data from file
        :return: a tuple of file name and limit order book data iterator
        :raise FileNotFoundError: if the file is not found
        """
        path = pathlib.Path(self._config.lob.indi.path)
        files = path.glob(self._config.lob.indi.pattern)

        for file in files:
            yield file.stem, self._preprocess_limit_order_book(file)

    def _preprocess_limit_order_book(self, file_path: pathlib.Path) -> pl.DataFrame:
        """
        Preprocess limit order book data
        :param file_path: file path
        :return: preprocessed data
        """
        def replace_prefix(old, new):
            new_cols = {name: name.replace(old, new) if name.startswith(old) else name for name in data.columns}
            return data.rename(new_cols)

        def replace_suffix(old, new):
            new_cols = {name: name.replace(old, new) if name.endswith(old) else name for name in data.columns}
            return data.rename(new_cols)

        # Load data
        data = pl.read_csv(file_path)

        # Prune some columns
        data: pl.DataFrame = data.drop(pl.selectors.matches("*_count"), "expected_price")

        # Filter data by time
        data = data.filter(
            (int(self._config.lob.start_time.strftime("%H%M%S")) * 100 <= pl.col("time"))
            & (pl.col("time") < int(self._config.lob.end_time.strftime("%H%M%S")) * 100)
        )

        # Normalize price
        data = data.with_columns(pl.selectors.matches("*_price") // 10)

        # shorten column names
        data = replace_prefix("bid_", "b")
        data = replace_prefix("ask_", "a")

        data = replace_suffix("_price", "p")
        data = replace_suffix("_amount", "a")

        # cast to int32
        data = data.cast(pl.Int32)

        return data

    def _load_price(self) -> Iterator[pl.DataFrame]:
        """
        Load price data from file
        :return: price data iterator
        :raise FileNotFoundError: if the file is not found
        """
        path = pathlib.Path(self._config.tr.indi.path)
        files = path.glob(self._config.tr.indi.pattern)

        for file in files:
            yield pl.read_csv(file)

