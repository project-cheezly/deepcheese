SELECT
    tr_date as date,
    value
FROM
    asset_history
INNER JOIN asset ON asset_history.asset_id = asset.id
INNER JOIN market ON asset.market_id = market.id
WHERE
    tr_date >= $1
AND
    market.kis_code = $2
AND
    asset.code = $3;
