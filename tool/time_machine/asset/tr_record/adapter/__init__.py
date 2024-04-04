from typing import Protocol

from ..tr_record_list import AssetTrRecordList


class AssetTrRecordDataSource(Protocol):
    def get_tr_record(self) -> AssetTrRecordList:
        pass
