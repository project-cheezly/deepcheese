SELECT
    tr.record_date as date,
    tr.category_id as category_id,
    market.kis_code as market_kis_code,
    asset.code as kis_code,
    tt.asset_flow as tr_type,
    tr.amount as amount
FROM transaction_record as tr
INNER JOIN asset ON tr.asset_id = asset.id
INNER JOIN market ON asset.market_id = market.id
INNER JOIN transaction_type as tt ON tr.tr_type_id = tt.id
ORDER BY tr.record_date ASC;
