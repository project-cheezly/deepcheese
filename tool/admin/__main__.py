import argparse
from .update_asset_list import update_domestic_asset, update_overseas_asset
from .update_asset_price import update_current_stock_price
from .update_currency import update_current_currency
from .update_category_history import update_category_value
from .time_machine import time_machine
from database_adapter import PostgreSQL
from kis_developer import KIS

parser = argparse.ArgumentParser(description='Update asset list and asset price')
parser.add_argument('--asset_list', action='store_true', help='Update asset list')
parser.add_argument('--asset_price', action='store_true', help='Update asset price')
parser.add_argument('--currency_value', action='store_true', help='Update currency value')
parser.add_argument('--update_category_history', action='store_true', help='Update category history')
parser.add_argument('--execute_time_machine', action='store_true', help='Execute time machine')

args = parser.parse_args()

db = PostgreSQL()
kis = KIS()

if args.execute_time_machine:
    time_machine(db, kis)
else:
    if args.asset_list:
        update_domestic_asset()
        update_overseas_asset()

    if args.asset_price:
        update_current_stock_price(db, kis)

    if args.currency_value:
        update_current_currency(db, kis)

    if args.update_category_history:
        update_category_value(db)

    db.conn.execute("INSERT INTO update_log (target, updated_timestamp) VALUES ('cheeseboard-cron', NOW()) ON CONFLICT (target) DO UPDATE SET updated_timestamp = NOW()")
    db.conn.commit()
