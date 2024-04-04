import datetime
import time

from kis_developer import KIS

from . import ChartHistoryDataSource
from ..history import ChartHistory
from ...asset.asset import AssetId, MarketCode
from ...currency.currency import CurrencyValue


class ChartHistoryKISAdapter(ChartHistoryDataSource):
    def __init__(self, kis: KIS):
        self.__kis = kis

    def get(self, date: datetime.date, asset_id: AssetId) -> ChartHistory:
        match asset_id.market_code:
            case MarketCode.KOSPI | MarketCode.KOSDAQ:
                result = self.__get_domestic_asset(date, asset_id)
            case (MarketCode.AMEX
                  | MarketCode.NASDAQ
                  | MarketCode.NYSE):
                result = self.__get_overseas_asset(date, asset_id)
            case _:
                raise NotImplementedError()

        return result

    def __get_domestic_asset(self, date: datetime.date, asset_id: AssetId) -> ChartHistory:
        time.sleep(0.2)
        date += datetime.timedelta(days=100)
        response = self.__kis.domestic.inquire_daily_price(asset_id.asset_code, date)

        if response["msg_cd"] != "MCA00000":
            raise ValueError(response["msg1"])

        result = ChartHistory()

        for data in response["output2"]:
            result.update(
                datetime.datetime.strptime(data["stck_bsop_date"], "%Y%m%d").date(),
                asset_id,
                CurrencyValue(data["stck_clpr"])
            )

        return result

    def __get_overseas_asset(self, date: datetime.date, asset_id: AssetId) -> ChartHistory:
        time.sleep(0.2)
        date += datetime.timedelta(days=100)
        response = self.__kis.overseas.inquire_daily_price(
            self.__parse_market_code(asset_id.market_code),
            asset_id.asset_code,
            date
        )

        if response["msg_cd"] != "MCA00000":
            raise ValueError(response["msg1"])

        result = ChartHistory()

        for data in response["output2"]:
            result.update(
                datetime.datetime.strptime(data["xymd"], "%Y%m%d").date(),
                asset_id,
                CurrencyValue(data["clos"])
            )

        return result

    @staticmethod
    def __parse_market_code(market_code: MarketCode) -> str:
        match market_code:
            case MarketCode.NYSE:
                return "NYS"
            case MarketCode.NASDAQ:
                return "NAS"
            case MarketCode.AMEX:
                return "AMS"
            case _:
                raise NotImplementedError()
