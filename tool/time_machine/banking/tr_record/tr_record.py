import datetime
from dataclasses import dataclass

from ...category.category import CategoryId
from time_machine.currency.currency import CurrencyType
from .. import BankingValue
from .tr_type import BankingTrType


@dataclass
class BankingTrRecord:
    date: datetime.date
    category_id: CategoryId
    currency_id: CurrencyType
    tr_type: BankingTrType
    value: BankingValue
