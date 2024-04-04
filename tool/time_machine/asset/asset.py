from enum import Enum
from typing import TypeAlias
from dataclasses import dataclass

AssetCode: TypeAlias = str
AssetAmount: TypeAlias = int


class MarketCode(Enum):
    KOSPI = 1
    KOSDAQ = 2
    NYSE = 3
    NASDAQ = 4
    AMEX = 5


@dataclass(frozen=True)
class AssetId:
    market_code: MarketCode
    asset_code: AssetCode


AssetPair: TypeAlias = dict[AssetId, AssetAmount]
