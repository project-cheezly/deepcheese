import datetime
import time

from database_adapter import PostgreSQL
from kis_developer import KIS

currency_update_query = """
    INSERT INTO currency_history (tr_date, currency_id, value)
    VALUES (%s, %s, %s)
    ON CONFLICT (tr_date, currency_id)
    DO UPDATE SET value = EXCLUDED.value
"""


def update_current_currency():
    database = PostgreSQL()
    kis = KIS()

    update_target_currencies = database.conn.execute("""
        SELECT currency.kis_code, currency.id
        FROM currency
        WHERE currency.code != 'KRW'
    """)

    targets = []

    for currency in update_target_currencies:
        kis_code = currency[0]
        day = (datetime.datetime.now() - datetime.timedelta(hours=5))

        if 9 > datetime.datetime.now().hour:
            continue

        value = kis.overseas.inquire_daily_forex(kis_code, day)['output2'][0]['ovrs_nmix_prpr']
        targets.append((day.date().strftime("%Y-%m-%d"), currency[1], value))

    database.conn.cursor().executemany(currency_update_query, targets)
    database.conn.commit()
