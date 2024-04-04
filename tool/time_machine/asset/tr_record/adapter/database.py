from pathlib import Path

from . import AssetTrRecordDataSource
from ...asset import AssetId, MarketCode
from ..tr_record import AssetTrRecord, AssetTrType
from ..tr_record_list import AssetTrRecordList
from ....category.category import CategoryId


class AssetTrRecordDatabaseAdapter(AssetTrRecordDataSource):
    def __init__(self, db):
        self.__db = db

    def get_tr_record(self) -> AssetTrRecordList:
        with open(Path(__file__).parent / 'load_tr_record.sql') as f:
            sql = f.read()
            response = self.__db.conn.execute(sql).fetchall()

        return AssetTrRecordList([self.__parse_to_asset_tr_record(row) for row in response])

    def __parse_to_asset_tr_record(self, row) -> AssetTrRecord:
        return AssetTrRecord(
            date=row[0],
            category_id=CategoryId(row[1]),
            asset_id=self.__parse_to_asset_id(row[2], row[3]),
            tr_type=self.__parse_to_tr_type(row[4]),
            amount=row[5]
        )

    def __parse_to_asset_id(self, market_code, asset_code):
        return AssetId(self.__parse_to_market_code(market_code), asset_code)

    @staticmethod
    def __parse_to_market_code(market_code):
        match market_code:
            case 'KOSPI':
                return MarketCode.KOSPI
            case 'KOSDAQ':
                return MarketCode.KOSDAQ
            case 'NYS':
                return MarketCode.NYSE
            case 'NAS':
                return MarketCode.NASDAQ
            case 'AMS':
                return MarketCode.AMEX
            case _:
                raise ValueError(f'Unknown market code: {market_code}')

    @staticmethod
    def __parse_to_tr_type(tr_type) -> AssetTrType:
        match tr_type:
            case 'INFLOW':
                return AssetTrType.INFLOW
            case 'OUTFLOW':
                return AssetTrType.OUTFLOW
            case _:
                raise ValueError(f'Unknown transaction type: {tr_type}')
