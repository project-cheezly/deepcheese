import typing

import polars as pl


class DataLoader(typing.Protocol):
    def lob_data(self) -> typing.Iterator[tuple[str, pl.DataFrame]]:
        """
        Load limit order book data
        :return: limit order book data iterator
        """
        pass

    def price_data(self) -> typing.Iterator[pl.DataFrame]:
        """
        Load price data
        :return: price data iterator
        """
        pass
