import datetime
from database_adapter import PostgreSQL
from kis_developer import KIS


def time_machine(db: PostgreSQL, kis: KIS):
    tr_record = db.conn.execute("""
        SELECT
            tr.record_date,
            tr.category_id,
            market.kis_code,
            market.currency_id,
            asset.code,
            tt.name as tr_type,
            tr.amount,
            tr.value
        FROM transaction_record as tr
        INNER JOIN asset ON tr.asset_id = asset.id
        INNER JOIN market ON asset.market_id = market.id
        INNER JOIN transaction_type as tt ON tr.tr_type_id = tt.id
        ORDER BY tr.record_date ASC;
    """).fetchall()

    bank_record = db.conn.execute("""
        SELECT
            record_date,
            category_id,
            currency_id,
            flow,
            value
        FROM bank_record
        ORDER BY record_date ASC;
    """).fetchall()

    currency_history = db.conn.execute("""
        SELECT
            tr_date,
            currency_id,
            value
        FROM currency_history;
    """).fetchall()

    current_asset = {}
    current_bank = {}
    asset_information = {}

    asset_history = {}
    bank_history = {}

    start_date = min(bank_record[0][0], tr_record[0][0])
    current_date = start_date

    tr_record_idx = 0
    bank_record_idx = 0

    while current_date <= datetime.datetime.now().date():
        while tr_record_idx < len(tr_record) and tr_record[tr_record_idx][0] == current_date:
            target = tr_record[tr_record_idx]

            category_id = target[1]
            kis_code = target[4]
            tr_type = target[5]
            amount = target[6]

            if current_asset.get(category_id) is None:
                current_asset[category_id] = {}

            if asset_information.get(kis_code) is None:
                asset_information[kis_code] = {
                    'currency_id': target[3],
                    'market_kis_code': target[2]
                }

            if current_asset[category_id].get(kis_code) is None:
                current_asset[category_id][kis_code] = 0

            match tr_type:
                case '매수':
                    current_asset[category_id][kis_code] += amount
                case '매도':
                    current_asset[category_id][kis_code] -= amount

            tr_record_idx += 1

        while bank_record_idx < len(bank_record) and bank_record[bank_record_idx][0] == current_date:
            target = bank_record[bank_record_idx]

            category_id = target[1]
            currency_id = target[2]
            flow = target[3]
            value = target[4]

            if current_bank.get(category_id) is None:
                current_bank[category_id] = {}

            if current_bank[category_id].get(currency_id) is None:
                current_bank[category_id][currency_id] = 0

            match flow:
                case 'INFLOW':
                    current_bank[category_id][currency_id] += value
                case 'OUTFLOW':
                    current_bank[category_id][currency_id] -= value

            bank_record_idx += 1

        for category_id, asset in current_asset.items():
            for kis_code, amount in asset.items():
                if asset_history.get(kis_code) is None:
                    asset_history[kis_code] = {}

                if asset_history[kis_code].get(current_date) is None:
                    data_from_db = db.conn.execute(f"""
                        SELECT
                            tr_date,
                            value
                        FROM asset_history
                        WHERE tr_date >= '{current_date}'
                        AND asset_id = {a};
                    """).fetchall()

        current_date += datetime.timedelta(days=1)


