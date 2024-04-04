import datetime

from ..asset.asset import AssetId
from ..currency.currency import CurrencyValue
from .history import ChartHistory
from .adapter.database import ChartHistoryDatabaseAdapter
from .adapter.kis import ChartHistoryKISAdapter


class ChartService:
    def __init__(self, db, kis):
        self.__chart_history = ChartHistory()
        self.__chart_data_sources: list[ChartHistory]
        self.__chart_data_source = ChartHistoryDatabaseAdapter(db)
        self.__chart_data_source_alt = ChartHistoryKISAdapter(kis)

    def get_chart(self, current_date: datetime.date, asset_id: AssetId) -> CurrencyValue:
        cached_result = self.__chart_history.get(current_date, asset_id)

        if cached_result is not None:
            return cached_result

        self.__chart_history += self.__chart_data_source.get(current_date, asset_id)
        result = self.__chart_history.get(current_date, asset_id)

        if result is not None:
            return result

        kis_response = self.__chart_data_source_alt.get(current_date, asset_id)
        self.__chart_history += kis_response

        result = self.__chart_history.get(current_date, asset_id)

        if result is not None:
            return result
        else:
            # todo
            raise NotImplementedError()
