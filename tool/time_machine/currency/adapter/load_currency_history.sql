SELECT
    tr_date as date,
    currency_id,
    value
FROM currency_history
WHERE tr_date >= %s
AND currency_id = %s
ORDER BY tr_date;
