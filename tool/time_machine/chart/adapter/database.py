import datetime
from pathlib import Path

from . import ChartHistoryDataSource, ChartHistoryDataSaver
from ..history import ChartHistory
from ...asset.asset import AssetId
from ...currency.currency import CurrencyValue


class ChartHistoryDatabaseAdapter(ChartHistoryDataSource):
    def __init__(self, db):
        self.__db = db

    def get(self, date: datetime.date, asset_id: AssetId) -> ChartHistory:
        with open(Path(__file__).parent / "load_chart_history.sql") as f:
            query = f.read()
            rows = self.__db.conn.execute(query, (date, asset_id.market_code, asset_id.asset_code)).fetchall()

        history = ChartHistory()
        for row in rows:
            history.update(row[0], asset_id, CurrencyValue(row[1]))

        return history


class ChartHistoryDatabaseSaver(ChartHistoryDataSaver):
    def __init__(self, db):
        self.__db = db
        self.asset_kv: dict[AssetId, int] = {}

    def save(self, history: ChartHistory):
        with open(Path(__file__).parent / "save_chart_history.sql") as f:
            query = f.read()

        query_values = []
        