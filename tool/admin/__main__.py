import argparse
from .update_asset_list import update_domestic_asset, update_overseas_asset
from .update_asset_price import update_current_stock_price
from .update_currency import update_current_currency

parser = argparse.ArgumentParser(description='Update asset list and asset price')
parser.add_argument('--asset_list', action='store_true', help='Update asset list')
parser.add_argument('--asset_price', action='store_true', help='Update asset price')
parser.add_argument('--currency_value', action='store_true', help='Update currency value')

args = parser.parse_args()

if args.asset_list:
    update_domestic_asset()
    update_overseas_asset()

if args.asset_price:
    update_current_stock_price()

if args.currency_value:
    update_current_currency()
