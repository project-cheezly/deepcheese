import datetime
from typing import Protocol

from ..currency_history import CurrencyHistory
from ..currency import CurrencyType


class CurrencyHistoryAdapter(Protocol):
    def get_history(self, currency: CurrencyType, target_date: datetime.date) -> CurrencyHistory:
        pass
