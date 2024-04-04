import datetime

from kis_developer import KIS
from database_adapter import PostgreSQL

from .banking.service import BankingService
from .asset.service import AssetService
from .category.category import CategoryId
from .currency.currency import CurrencyValue


class Machine:
    def __init__(self):
        self.__db = PostgreSQL()
        self.__kis = KIS()
        self.__banking_service = BankingService(self.__db, self.__kis)
        self.__asset_service = AssetService(self.__db, self.__kis)

    def run(self):
        current_date = self.__get_initial_date()
        end_date = datetime.datetime.now().date()

        result: list[tuple[datetime.date, CategoryId, CurrencyValue]] = []

        while current_date < end_date:
            asset_value = self.__asset_service.calculate_value(current_date)
            banking_value = self.__banking_service.calculate_value(current_date)

            updated_category = set(asset_value.keys()) | set(banking_value.keys())

            for category_id in updated_category:
                result.append((
                    current_date,
                    category_id,
                    asset_value.get(category_id, CurrencyValue(0))
                    + banking_value.get(category_id, CurrencyValue(0))
                ))

            current_date += datetime.timedelta(days=1)

        self.save(result)

    def __get_initial_date(self):
        return min(self.__banking_service.get_initial_date(), self.__asset_service.get_initial_date())

    def save(self, result):
        query = """
            INSERT INTO category_history (tr_date, category_id, value)
            VALUES (%s, %s, %s)
            ON CONFLICT (tr_date, category_id) DO UPDATE
            SET value = EXCLUDED.value
        """

        self.__db.conn.cursor().executemany(query, result)
        self.__db.conn.commit()
