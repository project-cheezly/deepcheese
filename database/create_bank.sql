CREATE OR REPLACE PROCEDURE insert_bank (
    p_record_date DATE,
    p_category_id INTEGER,
    p_account_id INTEGER,
    p_currency_id INTEGER,
    p_flow flow_type,
    p_value NUMERIC(12, 2)
)
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO bank_record
    (record_date, category_id, account_id, currency_id, flow, origin, value)
    VALUES (
        p_record_date,
        p_category_id,
        p_account_id,
        p_currency_id,
        p_flow,
        true,
        (
            CASE p_flow
            WHEN 'INFLOW' THEN p_value
            WHEN 'OUTFLOW' THEN -p_value
            END
        )
    );

    INSERT INTO bank_balance
    (category_id, account_id, currency_id, value)
    VALUES (
        p_category_id,
        p_account_id,
        p_currency_id,
        p_value
    )
    ON CONFLICT (category_id, account_id, currency_id)
    DO UPDATE SET
        value = bank_balance.value + (
            CASE p_flow
            WHEN 'INFLOW' THEN p_value
            WHEN 'OUTFLOW' THEN -p_value
            END
        );
END;
$$