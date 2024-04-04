import datetime
from typing import Protocol
from ...asset.asset import AssetId
from ..history import ChartHistory


class ChartHistoryDataSource(Protocol):
    def get(self, date: datetime.date, asset_id: AssetId) -> ChartHistory:
        pass


class ChartHistoryDataSaver(Protocol):
    def save(self, history: ChartHistory):
        pass
