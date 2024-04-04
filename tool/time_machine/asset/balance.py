from .asset import AssetPair, AssetAmount, AssetId
from .tr_record.tr_record import AssetTrRecord, AssetTrType
from ..category.category import CategoryId


class AssetBalance:
    def __init__(self):
        self.__data: dict[CategoryId, AssetPair] = {}

    def update(self, tr_list: list[AssetTrRecord]):
        for tr in tr_list:
            match tr.tr_type:
                case AssetTrType.INFLOW:
                    amount = tr.amount
                case AssetTrType.OUTFLOW:
                    amount = -tr.amount
                case _:
                    amount = 0

            self.__add(tr.category_id, tr.asset_id, amount)

    def __add(self, category_id: CategoryId, asset_id: AssetId, amount: AssetAmount):
        if category_id not in self.__data:
            self.__data[category_id] = {}

        if asset_id not in self.__data[category_id]:
            self.__data[category_id][asset_id] = amount
        else:
            self.__data[category_id][asset_id] += amount

    def get_current_balance(self) -> dict[CategoryId, AssetPair]:
        return self.__data
