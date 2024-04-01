CREATE OR REPLACE PROCEDURE delete_bank (
    p_bank_record_id INTEGER
)
LANGUAGE plpgsql
AS $$
DECLARE
    v_deleted_line bank_record;
BEGIN
    DELETE FROM bank_record
    WHERE id = p_bank_record_id
    RETURNING * INTO v_deleted_line;

    INSERT INTO bank_balance
    (category_id, account_id, currency_id, value)
    VALUES (
        v_deleted_line.category_id,
        v_deleted_line.account_id,
        v_deleted_line.currency_id,
        -v_deleted_line.value
    )
    ON CONFLICT (category_id, account_id, currency_id)
    DO UPDATE SET
        value = bank_balance.value - v_deleted_line.value;
END;
$$