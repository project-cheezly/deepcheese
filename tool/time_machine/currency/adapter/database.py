import datetime
from pathlib import Path

from . import CurrencyHistoryAdapter
from ..currency_history import CurrencyHistory
from ..currency import CurrencyType


class CurrencyHistoryDatabaseAdapter(CurrencyHistoryAdapter):
    def __init__(self, db):
        self.__db = db

    def get_history(self, currency: CurrencyType, target_date: datetime.date) -> CurrencyHistory:
        currency_id = self.__convert_to_currency_id(currency)
        with open(Path(__file__).parent / "load_currency_history.sql") as f:
            query = f.read()
            rows = self.__db.conn.execute(query, (target_date, currency_id)).fetchall()

        history = CurrencyHistory()
        for row in rows:
            currency_type = self.__convert_to_currency_type(row[1])
            history.update(row[0], currency_type, row[2])

        return history

    @staticmethod
    def __convert_to_currency_id(currency_type: CurrencyType) -> int:
        match currency_type:
            case CurrencyType.KRW:
                return 1
            case CurrencyType.USD:
                return 2
            case _:
                raise ValueError(f"Unsupported currency type: {currency_type}")

    @staticmethod
    def __convert_to_currency_type(currency_id: int) -> CurrencyType:
        match currency_id:
            case 1:
                return CurrencyType.KRW
            case 2:
                return CurrencyType.USD
            case _:
                raise ValueError(f"Unsupported currency id: {currency_id}")
