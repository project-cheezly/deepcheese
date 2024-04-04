import datetime
from typing import Self

from ..currency.currency import CurrencyValue
from ..asset.asset import AssetId


class ChartHistory:
    def __init__(self):
        self.__data: dict[datetime.date, dict[AssetId, CurrencyValue]] = {}
        self.__oldest_date: datetime.date = datetime.datetime.now().date() - datetime.timedelta(days=1)

    def get(self, date: datetime, asset_id: AssetId) -> CurrencyValue | None:
        if date not in self.__data or asset_id not in self.__data[date]:
            if date < self.__oldest_date:
                return None

            while date > self.__oldest_date:
                date -= datetime.timedelta(days=1)

                if date in self.__data:
                    if asset_id in self.__data[date]:
                        return self.__data[date][asset_id]

            return None

        return self.__data[date][asset_id]

    def update(self, date: datetime.date, asset_id: AssetId, value: CurrencyValue):
        if date not in self.__data:
            self.__data[date] = {}

        self.__oldest_date = min(self.__oldest_date, date)
        self.__data[date][asset_id] = value

    def __add__(self, other: Self) -> Self:
        for date, assets in other.__data.items():
            for asset, value in assets.items():
                self.update(date, asset, value)

        return self
