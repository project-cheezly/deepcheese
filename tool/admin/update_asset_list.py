import os
import pathlib
import httpx
import zipfile

from database_adapter import PostgreSQL

query = """
    INSERT INTO asset 
    (asset_type_id, market_id, code, name, update_disabled) 
    VALUES (%s, %s, %s, %s, %s)
    ON CONFLICT (asset_type_id, market_id, code) DO NOTHING;"""


def update_domestic_asset():
    __update_domestic_stock_list("kospi_code.mst", "코스피")
    __update_domestic_stock_list("kosdaq_code.mst", "코스닥")


def update_overseas_asset():
    for market in ["나스닥", "뉴욕증권거래소", "아멕스"]:
        __update_overseas_stock_list(market)


def __update_domestic_stock_list(file_name: str, korean_name: str):
    post = PostgreSQL()

    asset_type_id = post.conn.execute("SELECT id FROM asset_type WHERE name = '주식'").fetchone()[0]
    market_id = post.conn.execute(f"SELECT id FROM market WHERE name = '{korean_name}'").fetchone()[0]

    __download_mst(file_name)
    targets = []

    with open(file_name, 'r', encoding='cp949') as f:
        for line in f.readlines():
            rf1 = line[0:len(line) - 228]
            code = rf1[0:9].rstrip()
            name = rf1[21:].rstrip()

            targets.append((asset_type_id, market_id, code, name, False))

    post.conn.cursor().executemany(query, targets)
    post.conn.commit()

    __cleanup(file_name)


def __update_overseas_stock_list(korean_name: str):
    post = PostgreSQL()

    asset_type_id = post.conn.execute("SELECT id FROM asset_type WHERE name = '주식'").fetchone()[0]
    market_id, market_code = (post
                              .conn
                              .execute(f"SELECT id, kis_code FROM market WHERE name = '{korean_name}'")
                              .fetchone())
    file_name = f"{market_code}mst.cod"

    __download_mst(file_name)
    targets = []

    with open(file_name, 'r', encoding='cp949') as f:
        for line in f.readlines():
            data = line.split('\t')
            code = data[4].rstrip()
            name = data[6].rstrip()[:50]

            targets.append((asset_type_id, market_id, code, name, False))

    post.conn.cursor().executemany(query, targets)
    post.conn.commit()

    __cleanup(file_name)


def __download_mst(file_name):
    binary_response = httpx.get(f"https://new.real.download.dws.co.kr/common/master/{file_name}.zip").content
    with open(pathlib.Path('.') / f'{file_name}.zip', 'wb') as f:
        f.write(binary_response)
    with zipfile.ZipFile(pathlib.Path('.') / f'{file_name}.zip') as zip_ref:
        zip_ref.extractall()
    try:
        os.remove(f"{file_name}.zip")
    except FileNotFoundError:
        pass


def __cleanup(file_name):
    try:
        os.remove(file_name)
    except FileNotFoundError:
        pass
