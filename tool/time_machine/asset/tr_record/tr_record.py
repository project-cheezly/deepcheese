import datetime
from enum import Enum
from dataclasses import dataclass

from ..asset import AssetId, AssetAmount
from ...category.category import CategoryId


@dataclass
class AssetTrType:
    INFLOW = 1
    OUTFLOW = 2


@dataclass
class AssetTrRecord:
    date: datetime.date
    category_id: CategoryId
    asset_id: AssetId
    tr_type: AssetTrType
    amount: AssetAmount
