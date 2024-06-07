WITH AssetValueOrderedDate AS (
    SELECT
        asset_id,
        value,
        tr_date,
        ROW_NUMBER() OVER (PARTITION BY asset_id ORDER BY tr_date DESC) AS rn
    FROM
        asset_history
),
AssetLatestValue AS (
    SELECT
        asset_id,
        value
    FROM
        AssetValueOrderedDate
    WHERE
        rn = 1
),
CurrencyValueOrderedDate AS (
    SELECT
        currency_id,
        value,
        ROW_NUMBER() OVER (PARTITION BY currency_id ORDER BY tr_date DESC) AS rn
    FROM
        currency_history
),
CurrencyLatestValue AS (
    SELECT
        currency_id,
        value
    FROM
        CurrencyValueOrderedDate
    WHERE
        rn = 1
),
CurrentCategoryValue AS (
    SELECT
        asset_balance.category_id,
        sum(asset_balance.amount * AssetLatestValue.value
        * (CASE
            WHEN market.currency_id = 1 THEN 1
            ELSE CurrencyLatestValue.value
            END)) AS value
    FROM asset_balance
    INNER JOIN AssetLatestValue
        ON asset_balance.asset_id = AssetLatestValue.asset_id
    INNER JOIN asset
        ON asset_balance.asset_id = asset.id
    INNER JOIN market
        ON asset.market_id = market.id
    LEFT JOIN CurrencyLatestValue
        ON market.currency_id = CurrencyLatestValue.currency_id
    GROUP BY asset_balance.category_id
),
CurrentCategoryBankValue AS (
    SELECT
        bank_balance.category_id,
        sum(bank_balance.value
        * (CASE
            WHEN bank_balance.currency_id = 1 THEN 1
            ELSE CurrencyLatestValue.value
            END)) AS value
    FROM bank_balance
    LEFT OUTER JOIN CurrencyLatestValue
        ON bank_balance.currency_id = CurrencyLatestValue.currency_id
    GROUP BY bank_balance.category_id
),
I AS (
    INSERT INTO realtime_category_history (tr_timestamp, category_id, value)
    SELECT
        CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Seoul',
        COALESCE(CurrentCategoryValue.category_id, CurrentCategoryBankValue.category_id) AS category_id,
        COALESCE(CurrentCategoryValue.value, 0) + COALESCE(CurrentCategoryBankValue.value, 0) AS value
    FROM
        CurrentCategoryValue
    FULL OUTER JOIN CurrentCategoryBankValue
        ON CurrentCategoryValue.category_id = CurrentCategoryBankValue.category_id
)
INSERT INTO category_history (tr_date, category_id, value)
SELECT
    CURRENT_DATE,
    COALESCE(CurrentCategoryValue.category_id, CurrentCategoryBankValue.category_id) AS category_id,
    COALESCE(CurrentCategoryValue.value, 0) + COALESCE(CurrentCategoryBankValue.value, 0) AS value
FROM
    CurrentCategoryValue
FULL OUTER JOIN CurrentCategoryBankValue
    ON CurrentCategoryValue.category_id = CurrentCategoryBankValue.category_id
ON CONFLICT (tr_date, category_id) DO UPDATE SET value = EXCLUDED.value;
