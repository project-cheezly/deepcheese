import datetime

from .asset import MarketCode
from .tr_record.adapter.database import AssetTrRecordDatabaseAdapter
from ..currency.currency import CurrencyValue, CurrencyType
from ..currency.service import CurrencyService
from ..category.category import CategoryId
from .balance import AssetBalance, AssetId
from ..chart.service import ChartService


class AssetService:
    def __init__(self, db, kis):
        self.__chart_service = ChartService(db, kis)
        self.__currency_service = CurrencyService(db, kis)

        asset_tr_data_source = AssetTrRecordDatabaseAdapter(db)
        self.__asset_tr_record = asset_tr_data_source.get_tr_record()

        self.__balance = AssetBalance()

    def get_initial_date(self):
        return self.__asset_tr_record.get_initial_date()

    def calculate_value(self, current_date: datetime.date) -> dict[CategoryId, CurrencyValue]:
        tr_list = self.__asset_tr_record.get_tr_list(current_date)
        self.__balance.update(tr_list)

        current_balance = self.__balance.get_current_balance()

        result: dict[CategoryId, CurrencyValue] = {}

        for category_id, asset_pair in current_balance.items():
            for asset_id, amount in asset_pair.items():
                value = self.__chart_service.get_chart(current_date, asset_id)

                currency_id = self.get_currency_id(asset_id)
                currency_ratio = self.__currency_service.get_currency(current_date, currency_id)

                if category_id not in result:
                    result[category_id] = CurrencyValue(0)

                result[category_id] += value * amount * currency_ratio

        return result

    @staticmethod
    def get_currency_id(asset_id: AssetId) -> CurrencyType:
        match asset_id.market_code:
            case (
                MarketCode.KOSPI
                | MarketCode.KOSDAQ
            ):
                return CurrencyType.KRW
            case (
                MarketCode.NYSE
                | MarketCode.NASDAQ
                | MarketCode.AMEX
            ):
                return CurrencyType.USD
