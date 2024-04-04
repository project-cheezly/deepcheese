import datetime
import time

from kis_developer import KIS

from . import CurrencyHistoryAdapter
from ..currency import CurrencyType, CurrencyValue
from ..currency_history import CurrencyHistory


class CurrencyHistoryKISAdapter(CurrencyHistoryAdapter):
    def __init__(self, kis: KIS):
        self.__kis = kis

    def get_history(self, currency: CurrencyType, target_date: datetime.date) -> CurrencyHistory:
        currency_id = self.__convert_to_currency_id(currency)
        time.sleep(0.2)
        target_date += datetime.timedelta(days=100)
        response = self.__kis.overseas.inquire_daily_forex(currency_id, target_date)

        if response["msg_cd"] != "MCA00000":
            raise ValueError(response["msg1"])

        result = CurrencyHistory()

        for data in response["output2"]:
            result.update(
                datetime.datetime.strptime(data["stck_bsop_date"], "%Y%m%d").date(),
                currency,
                CurrencyValue(data["ovrs_nmix_prpr"])
            )

        return result

    @staticmethod
    def __convert_to_currency_id(currency_type: CurrencyType) -> str:
        match currency_type:
            case CurrencyType.USD:
                return "FX@KRW"
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
