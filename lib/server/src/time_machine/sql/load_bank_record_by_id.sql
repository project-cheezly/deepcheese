SELECT
    br.record_date as date,
    br.category_id,
    cu.code as currency_type,
    CAST(br.flow as TEXT) as tr_type,
    br.value AS value
FROM bank_record AS br
INNER JOIN currency AS cu ON cu.id = br.currency_id
WHERE br.id = $1;