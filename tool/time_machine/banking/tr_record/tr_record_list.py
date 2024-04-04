import datetime

from .tr_record import BankingTrRecord


class BankingTrRecordList:
    def __init__(self, data: list[BankingTrRecord] = None):
        if data is None:
            data = []

        self.__data: dict[datetime.date, list[BankingTrRecord]] = {}
        for tr in data:
            self.__data.setdefault(tr.date, []).append(tr)

    def get_tr_list(self, date: datetime.date) -> list[BankingTrRecord]:
        return self.__data.get(date, [])

    def get_initial_date(self) -> datetime.date:
        return min(self.__data.keys())
