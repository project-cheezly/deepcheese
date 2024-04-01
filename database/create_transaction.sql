CREATE OR REPLACE PROCEDURE insert_transaction (
    p_record_date DATE,
    p_category_id INTEGER,
    p_account_id INTEGER,
    p_asset_id INTEGER,
    p_tr_type_id INTEGER,
    p_amount INTEGER,
    p_value NUMERIC(12, 2),
    p_fee NUMERIC(12, 2)
)
LANGUAGE plpgsql
AS $$
DECLARE
    bank_tr_id INTEGER;
    v_bank_flow flow_type;
    v_asset_flow flow_type;
    v_currency_id INTEGER;
BEGIN
    v_bank_flow := (SELECT bank_flow FROM transaction_type WHERE id = p_tr_type_id);
    v_asset_flow := (SELECT asset_flow FROM transaction_type WHERE id = p_tr_type_id);
    v_currency_id := (
        SELECT market.currency_id
        FROM asset
        INNER JOIN market ON asset.market_id = market.id
        WHERE asset.id = p_asset_id
    );

    IF v_bank_flow <> 'CONSISTENT' THEN
        INSERT INTO bank_record
        (record_date, category_id, account_id, currency_id, flow, origin, value)
        VALUES (p_record_date,
                p_category_id,
                p_account_id,
                v_currency_id,
                v_bank_flow,
                false,
                p_value * p_amount
                + (CASE v_bank_flow
                    WHEN 'INFLOW' THEN -p_fee
                    WHEN 'OUTFLOW' THEN p_fee
                    ELSE 0
                    END)
        ) RETURNING id INTO STRICT bank_tr_id;
    END IF;

    INSERT INTO transaction_record
    (record_date, category_id, account_id, asset_id, tr_type_id, amount, value, fee, bank_record_id)
    VALUES (
        p_record_date,
        p_category_id,
        p_account_id,
        p_asset_id,
        p_tr_type_id,
        p_amount,
        p_value,
        p_fee,
        bank_tr_id
    );

    IF v_asset_flow <> 'CONSISTENT' THEN
        INSERT INTO asset_balance
        (category_id, account_id, asset_id, amount)
        VALUES (
            p_category_id,
            p_account_id,
            p_asset_id,
            (CASE v_asset_flow
                 WHEN 'INFLOW' THEN p_amount
                 WHEN 'OUTFLOW' THEN -p_amount
                 ELSE 0
            END)
        )
        ON CONFLICT (category_id, account_id, asset_id)
        DO UPDATE SET
        amount = asset_balance.amount
            + (CASE v_asset_flow
                WHEN 'INFLOW' THEN p_amount
                WHEN 'OUTFLOW' THEN -p_amount
                ELSE 0
            END);
    END IF;

    IF v_bank_flow <> 'CONSISTENT' THEN
        INSERT INTO bank_balance
        (category_id, account_id, currency_id, value)
        VALUES (
            p_category_id,
            p_account_id,
            v_currency_id,
            (
                CASE v_bank_flow
                WHEN 'INFLOW' THEN p_value
                WHEN 'OUTFLOW' THEN -p_value
                ELSE 0 END
            ) * p_amount - p_fee
        )
        ON CONFLICT (category_id, account_id, currency_id)
        DO UPDATE SET
            value = bank_balance.value
                + (
                    CASE v_bank_flow
                    WHEN 'INFLOW' THEN p_value
                    WHEN 'OUTFLOW' THEN -p_value
                    ELSE 0 END
                ) * p_amount - p_fee;
    END IF;
end;
$$;