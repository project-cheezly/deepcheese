from ..category.category import CategoryId
from time_machine.currency.currency import CurrencyValue, CurrencyType, CurrencyPair
from .tr_record.tr_record import BankingTrRecord, BankingTrType


class BankBalance:
    def __init__(self):
        self.__data: dict[CategoryId, CurrencyPair] = {}

    def update(self, tr_list: list[BankingTrRecord]):
        for tr in tr_list:
            match tr.tr_type:
                case BankingTrType.INFLOW:
                    self.__add(tr.category_id, tr.currency_id, tr.value)
                case BankingTrType.OUTFLOW:
                    self.__add(tr.category_id, tr.currency_id, -tr.value)

    def __add(self, category_id: CategoryId, currency_type: CurrencyType, value: CurrencyValue):
        if category_id not in self.__data:
            self.__data[category_id] = {}

        if currency_type not in self.__data[category_id]:
            self.__data[category_id][currency_type] = value
        else:
            self.__data[category_id][currency_type] += value

    def get_current_balance(self) -> dict[CategoryId, CurrencyPair]:
        return self.__data
