import datetime

from .tr_record.adapter.database import BankTrRecordDatabaseAdapter
from .balance import BankBalance
from time_machine.currency.currency import CurrencyValue
from time_machine.currency.service import CurrencyService
from ..category.category import CategoryId


class BankingService:
    def __init__(self, db, kis):
        self.__currency_service = CurrencyService(db, kis)

        self.__balance = BankBalance()

        banking_data_source = BankTrRecordDatabaseAdapter(db)
        self.__banking_tr_record = banking_data_source.get_tr()

    def get_initial_date(self):
        return self.__banking_tr_record.get_initial_date()

    def calculate_value(self, current_date: datetime.date) -> dict[CategoryId, CurrencyValue]:
        tr_list = self.__banking_tr_record.get_tr_list(current_date)
        self.__balance.update(tr_list)

        current_balance = self.__balance.get_current_balance()

        result: dict[CategoryId, CurrencyValue] = {}

        for category_id, currency_pair in current_balance.items():
            for currency_type, value in currency_pair.items():
                ratio = self.__currency_service.get_currency(current_date, currency_type)

                if category_id not in result:
                    result[category_id] = value * ratio
                else:
                    result[category_id] += value * ratio

        return result
