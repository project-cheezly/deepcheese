import datetime
import time

from database_adapter import PostgreSQL
from kis_developer import KIS

price_update_query = """
    INSERT INTO asset_history (tr_date, asset_id, value)
    VALUES (%s, %s, %s)
    ON CONFLICT (tr_date, asset_id)
    DO UPDATE SET value = EXCLUDED.value
"""


def update_current_stock_price():
    database = PostgreSQL()
    kis = KIS()

    update_target_assets = database.conn.execute("""
        SELECT market.kis_code, asset.code, asset.id
        FROM asset
        INNER JOIN asset_type ON asset.asset_type_id = asset_type.id
        INNER JOIN market ON asset.market_id = market.id
        INNER JOIN (
            SELECT asset_id, SUM(amount)
            FROM asset_balance
            GROUP BY asset_id
        ) AS ab ON asset.id = ab.asset_id
        WHERE asset_type.name = '주식'
        AND ab.sum > 0
    """)

    targets = []

    for asset in update_target_assets:
        kis_code = asset[0]
        code = asset[1]
        asset_id = asset[2]

        day = (datetime.datetime.now() - datetime.timedelta(hours=5)).date().strftime("%Y-%m-%d")

        match kis_code:
            case 'KOSPI' | 'KOSDAQ':
                if datetime.datetime.now().hour > 16 or datetime.datetime.now().hour < 9:
                    continue
                stock_price = kis.domestic.inquire_price(code)['output']['stck_prpr']
            case _:
                if 5 < datetime.datetime.now().hour < 22:
                    continue
                stock_price = kis.overseas.inquire_price(kis_code, code)['output']['last']

        targets.append((day, asset_id, stock_price))
        time.sleep(1)

    database.conn.cursor().executemany(price_update_query, targets)
    database.conn.commit()
