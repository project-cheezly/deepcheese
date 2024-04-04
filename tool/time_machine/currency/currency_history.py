import datetime
from typing import Self

from .currency import CurrencyType, CurrencyValue, CurrencyPair


class CurrencyHistory:
    def __init__(self):
        self.__data: dict[datetime.date, CurrencyPair] = {}
        self.__oldest_date: datetime.date = datetime.date.max

    def get(self, date: datetime.date, currency: CurrencyType) -> CurrencyValue | None:
        if date not in self.__data:
            if date < self.__oldest_date:
                return None

            while date not in self.__data:
                date -= datetime.timedelta(days=1)

            if currency not in self.__data[date]:
                return None
            else:
                return self.__data[date][currency]

        if currency not in self.__data[date]:
            return None

        return self.__data[date][currency]

    def update(self, date: datetime.date, currency: CurrencyType, value: CurrencyValue):
        if date not in self.__data:
            self.__data[date] = {}
        self.__data[date][currency] = value
        self.__oldest_date = min(self.__oldest_date, date)

    def __add__(self, other: Self) -> Self:
        for date, currencies in other.__data.items():
            for currency, value in currencies.items():
                self.update(date, currency, value)

        return self
