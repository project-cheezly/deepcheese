use chrono::Timelike;
use futures::future::join_all;
use kis::KIS;
use sqlx::{Executor, PgPool, QueryBuilder, Row};

const UPDATE_CURRENCY_VALUE_QUERY: &str = r#"
INSERT INTO currency_history (tr_date, currency_id, value)
"#;

const GET_TARGET_QUERY: &str = r#"
SELECT currency.kis_code, currency.id
FROM currency
WHERE currency.code != 'KRW'
"#;

#[derive(Debug)]
struct Currency {
    code: String,
    id: i32,
}

#[derive(Debug)]
struct CurrencyValue {
    id: i32,
    value: f64,
}

pub async fn update_currency_value(pool: &PgPool, kis: &KIS)
    -> Result<(), Box<dyn std::error::Error>>
{
    let current_time = chrono::offset::Utc::now();
    if 9 > current_time.hour() && current_time.hour() > 15 {
        return Ok(());
    }

    let mut conn = pool.acquire().await?;

    let currency_values = join_all(conn.fetch_all(GET_TARGET_QUERY).await?
        .into_iter()
        .map(|row| {
            Currency {
                code: row.get::<String, _>(0),
                id: row.get::<i32, _>(1),
            }
        }).map(|curr| get_currency_value(curr, kis)).collect::<Vec<_>>()).await;

    let mut builder = QueryBuilder::new(UPDATE_CURRENCY_VALUE_QUERY);

    builder.push_values(currency_values.iter(), |mut b, record| {
        if let Ok(rec) = record {
            b.push_bind(chrono::offset::Utc::now().date_naive())
                .push_bind(rec.id)
                .push_bind(rec.value);
        }
    });

    builder.push(r#"
        ON CONFLICT (tr_date, currency_id) DO UPDATE
        SET value = EXCLUDED.value
    "#);

    builder.build().execute(pool).await?;

    Ok(())
}

async fn get_currency_value(currency: Currency, kis: &KIS) -> Result<CurrencyValue, Box<dyn std::error::Error>> {
    let value = kis.overseas.inquire_daily_forex_value(
        &currency.code,
        chrono::offset::Utc::now().date_naive(),
        chrono::offset::Utc::now().date_naive()).await?;

    Ok(CurrencyValue {
        id: currency.id,
        value: value.get(chrono::offset::Utc::now().date_naive()).unwrap().종가,
    })
}