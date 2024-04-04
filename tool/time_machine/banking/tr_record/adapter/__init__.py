from typing import Protocol

from ..tr_record_list import BankingTrRecordList


class BankingTrAdapter(Protocol):
    def get_tr(self) -> BankingTrRecordList:
        pass
