from pathlib import Path

from . import BankingTrAdapter
from time_machine.currency.currency import CurrencyType
from ..tr_record_list import BankingTrRecordList
from ..tr_record import BankingTrRecord, BankingTrType


class BankTrRecordDatabaseAdapter(BankingTrAdapter):
    def __init__(self, db):
        self.__db = db

    def get_tr(self) -> BankingTrRecordList:
        with open(Path(__file__).parent / "load_bank_record.sql") as f:
            query = f.read()
            rows = self.__db.conn.execute(query).fetchall()

        tr_list = []
        for row in rows:
            tr = BankingTrRecord(
                date=row[0],
                category_id=row[1],
                currency_id=self.__convert_to_currency_type(row[2]),
                tr_type=self.__convert_to_tr_type(row[3]),
                value=row[4]
            )
            tr_list.append(tr)

        return BankingTrRecordList(tr_list)

    @staticmethod
    def __convert_to_currency_type(currency_id: int) -> CurrencyType:
        match currency_id:
            case 1:
                return CurrencyType.KRW
            case 2:
                return CurrencyType.USD
            case _:
                raise ValueError(f"Unsupported currency id: {currency_id}")

    @staticmethod
    def __convert_to_tr_type(tr_code: str) -> BankingTrType:
        match tr_code:
            case "INFLOW":
                return BankingTrType.INFLOW
            case "OUTFLOW":
                return BankingTrType.OUTFLOW
            case _:
                raise ValueError(f"Unsupported tr code: {tr_code}")
