SELECT
    record_date as date,
    category_id,
    currency_id,
    flow as tr_type,
    value
FROM bank_record
ORDER BY record_date ASC;
