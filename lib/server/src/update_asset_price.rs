use std::str::FromStr;
use kis::{KIS, MarketCode};
use sqlx::{Executor, PgPool, QueryBuilder, Row};
use futures::future::join_all;
use crate::config::{DomesticMarket, OverseasMarket};

const UPDATE_PRICE_QUERY: &str = r#"
    INSERT INTO asset_history (tr_date, asset_id, value)
"#;

const UPDATE_PRICE_CONFLICT_QUERY: &str = r#"
    ON CONFLICT (tr_date, asset_id)
    DO UPDATE SET value = EXCLUDED.value;
"#;

const GET_STOCK_CODE_QUERY: &str = r#"
    SELECT market.kis_code, asset.code, asset.id
    FROM asset
    INNER JOIN asset_type ON asset.asset_type_id = asset_type.id
    INNER JOIN market ON asset.market_id = market.id
    INNER JOIN (
        SELECT asset_id, SUM(amount)
        FROM asset_balance
        GROUP BY asset_id
    ) AS ab ON asset.id = ab.asset_id
    WHERE asset_type.name = '주식'
    AND ab.sum > 0
"#;

#[derive(Debug)]
struct Asset {
    market_code: String,
    stock_code: String,
    asset_id: i32
}

#[derive(Debug)]
struct AssetValue {
    asset_id: i32,
    asset_value: f64
}

pub async fn update_current_stock_price(pool: &PgPool, kis: &KIS)
    -> Result<(), Box<dyn std::error::Error>>
{
    let mut conn = pool.acquire().await?;

    let query_result = conn.fetch_all(GET_STOCK_CODE_QUERY).await?;
    let asset_values = join_all(query_result.into_iter().map(|x| {
        Asset {
            market_code: x.get::<String, _>(0),
            stock_code: x.get::<String, _>(1),
            asset_id: x.get::<i32, _>(2)
        }
    }).map(|x| get_current_price(x, kis)).collect::<Vec<_>>())
        .await
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut builder = QueryBuilder::new(UPDATE_PRICE_QUERY);

    builder.push_values(asset_values, |mut b, record| {
        b.push_bind(chrono::offset::Local::now().date_naive())
            .push_bind(record.asset_id)
            .push_bind(record.asset_value);
    }).push(UPDATE_PRICE_CONFLICT_QUERY);

    let _ = builder.build().execute(pool).await?;

    Ok(())
}

async fn get_current_price(asset: Asset, kis: &KIS) -> Option<AssetValue> {
    if MarketCode::from_str(&asset.market_code).is_err() {
        get_current_domestic_price(asset, kis).await
    } else {
        get_current_overseas_price(asset, kis).await
    }
}

async fn get_current_domestic_price(asset: Asset, kis: &KIS) -> Option<AssetValue> {
    if DomesticMarket::is_open(chrono::offset::Utc::now()) {
        return None;
    }

    match kis.domestic.inquire_stock_price(&asset.stock_code).await {
        Ok(x) => {
            Some(AssetValue {
                asset_id: asset.asset_id,
                asset_value: x.현재가 as f64
            })
        },
        Err(_) => None,
    }
}

async fn get_current_overseas_price(asset: Asset, kis: &KIS) -> Option<AssetValue> {
    if OverseasMarket::is_open(chrono::offset::Utc::now()) {
        return None;
    }

    match kis.overseas.inquire_stock_price(
        &asset.stock_code,
        MarketCode::from_str(&asset.market_code).unwrap()).await
    {
        Ok(x) => {
            Some(AssetValue {
                asset_id: asset.asset_id,
                asset_value: x.현재가
            })
        },
        Err(_) => None
    }
}