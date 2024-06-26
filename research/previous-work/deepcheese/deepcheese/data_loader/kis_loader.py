import pathlib
import logging
from typing import Iterator

import polars as pl

from deepcheese.data_loader import DataLoader
from static_conf import Config


logger = logging.getLogger(__name__)


class KisLoader(DataLoader):
    """
    Kis dataset loader
    """
    def __init__(self):
        self._config = Config().data_loader

    def lob_data(self) -> Iterator[tuple[str, pl.DataFrame]]:
        """
        Load limit order book data
        :return: A generator of a tuple containing the file name and the preprocessed data
        """
        return self._load_limit_order_book()

    def price_data(self) -> Iterator[pl.DataFrame]:
        return self._load_price()

    def _load_limit_order_book(self) -> Iterator[tuple[str, pl.DataFrame]]:
        """
        Load limit order book data from file
        :return: A tuple containing the file name and the preprocessed data
        :raise FileNotFoundError: if the file is not found
        """
        path = pathlib.Path(self._config.lob.kis.path)
        files = path.glob(self._config.lob.kis.pattern)

        for file in files:
            logger.debug(f"loading {file}")
            yield self._preprocess_limit_order_book(file)

    def _preprocess_limit_order_book(self, file_path: pathlib.Path) -> tuple[str, pl.DataFrame]:
        """
        Preprocess limit order book data
        :param file_path: file path
        :return: A tuple containing the file name and the preprocessed data
        """
        # Load data
        new_columns = ["_0", "time"]
        new_columns.extend([f"b{i}p" for i in range(1, 6)])
        new_columns.extend([f"a{i}p" for i in range(1, 6)])
        new_columns.extend([f"b{i}a" for i in range(1, 6)])
        new_columns.extend([f"a{i}a" for i in range(1, 6)])
        new_columns.extend([f"_{i}" for i in range(1, 17)])
        data = pl.read_csv(file_path, has_header=False, new_columns=new_columns)

        # Prune some columns
        data: pl.DataFrame = data.drop(pl.selectors.matches("*_[0-9]"))

        # Filter data by time
        data = data.filter(
            (int(self._config.lob.start_time.strftime("%H%M%S")) <= pl.col("time"))
            & (pl.col("time") < int(self._config.lob.end_time.strftime("%H%M%S")))
        )
        data = data.with_columns(pl.col("time") * 100)

        # Filter zero price
        data = data.filter(pl.col("b1p") > 0)

        # Normalize price
        data = data.with_columns(pl.selectors.matches("*p") * 10)
        data = data.cast(pl.Int32)

        return file_path.stem, data

    def _load_price(self) -> Iterator[pl.DataFrame]:
        """
        Load price data from file
        :return: price data
        :raise FileNotFoundError: if the file is not found
        """
        path = pathlib.Path(self._config.tr.kis.path)
        files = path.glob(self._config.tr.kis.pattern)

        logger.info(files)

        for file in files:
            yield pl.read_csv(file)
