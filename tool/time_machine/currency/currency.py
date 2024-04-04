from typing import TypeAlias
from enum import Enum
from decimal import Decimal


class CurrencyType(Enum):
    KRW = 1
    USD = 2


CurrencyValue: TypeAlias = Decimal
CurrencyPair: TypeAlias = dict[CurrencyType, CurrencyValue]
