import pathlib
import datetime
from database_adapter import PostgreSQL


def update_category_value(db: PostgreSQL):
    day = (datetime.datetime.now() - datetime.timedelta(hours=5)).date().strftime("%Y-%m-%d")
    db.conn.cursor().execute(open(pathlib.Path(__file__).parent / 'update_category_history.sql').read(), (day,))
    db.conn.commit()
