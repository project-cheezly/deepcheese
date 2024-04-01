CREATE OR REPLACE PROCEDURE delete_transaction (
    p_transaction_id IN transaction_record.id%TYPE
)
LANGUAGE plpgsql
AS $$
DECLARE
    v_deleted_line transaction_record;
    v_bank_flow transaction_type.bank_flow%TYPE;
    v_bank_currency_id bank_record.currency_id%TYPE;
    v_bank_value bank_record.value%TYPE;
    v_asset_flow transaction_type.asset_flow%TYPE;
BEGIN
    DELETE FROM transaction_record
    WHERE id = p_transaction_id
    RETURNING * INTO v_deleted_line;

    IF v_deleted_line IS NULL THEN
        RAISE EXCEPTION 'Transaction with id % not found', p_transaction_id;
    END IF;

    IF v_deleted_line.bank_record_id IS NOT NULL THEN
        DELETE FROM bank_record
        WHERE id = v_deleted_line.bank_record_id
        RETURNING flow, value, currency_id INTO STRICT v_bank_flow, v_bank_value, v_bank_currency_id;
    END IF;

    v_asset_flow := (SELECT asset_flow FROM transaction_type WHERE id = v_deleted_line.tr_type_id);
    IF v_asset_flow <> 'CONSISTENT' THEN
        INSERT INTO asset_balance
        (category_id, account_id, asset_id, amount)
        VALUES (
            v_deleted_line.category_id,
            v_deleted_line.account_id,
            v_deleted_line.asset_id,
            CASE v_asset_flow
                WHEN 'INFLOW' THEN -v_deleted_line.amount
                WHEN 'OUTFLOW' THEN v_deleted_line.amount
            END
        )
        ON CONFLICT (category_id, account_id, asset_id)
        DO UPDATE SET
            amount = asset_balance.amount
                + (CASE v_asset_flow
                WHEN 'INFLOW' THEN -v_deleted_line.amount
                WHEN 'OUTFLOW' THEN v_deleted_line.amount
                END);
    END IF;

    IF v_bank_flow <> 'CONSISTENT' THEN
        INSERT INTO bank_balance
        (category_id, account_id, currency_id, value)
        VALUES (
            v_deleted_line.category_id,
            v_deleted_line.account_id,
            v_bank_currency_id,
            CASE v_bank_flow
                WHEN 'INFLOW' THEN -v_bank_value
                WHEN 'OUTFLOW' THEN v_bank_value
            END
        )
        ON CONFLICT (category_id, account_id, currency_id)
        DO UPDATE SET
            value = bank_balance.value
                + (CASE v_bank_flow
                WHEN 'INFLOW' THEN -v_bank_value
                WHEN 'OUTFLOW' THEN v_bank_value
                END);
    end if;
END;
$$;