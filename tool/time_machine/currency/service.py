import datetime

from .currency import CurrencyValue, CurrencyType
from .currency_history import CurrencyHistory
from .adapter.database import CurrencyHistoryDatabaseAdapter
from .adapter.kis import CurrencyHistoryKISAdapter


class CurrencyService:
    def __init__(self, db, kis):
        self.__currency_history = CurrencyHistory()
        self.__currency_data_source = CurrencyHistoryDatabaseAdapter(db)
        self.__currency_data_source_alt = CurrencyHistoryKISAdapter(kis)

    def get_currency(self, current_date: datetime.date, currency_type: CurrencyType) -> CurrencyValue:
        if currency_type == CurrencyType.KRW:
            return CurrencyValue(1)

        cached_result = self.__currency_history.get(current_date, currency_type)

        if cached_result is not None:
            return cached_result

        self.__currency_history += self.__currency_data_source.get_history(currency_type, current_date)
        result = self.__currency_history.get(current_date, currency_type)

        if result is not None:
            return result

        self.__currency_history += self.__currency_data_source_alt.get_history(currency_type, current_date)
        result = self.__currency_history.get(current_date, currency_type)

        if result is not None:
            return result
        else:
            # todo 과거 거 끌어와야함
            raise NotImplementedError()
