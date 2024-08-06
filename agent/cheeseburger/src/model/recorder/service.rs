use std::sync::Arc;
use std::time::Duration;
use sqlx::{Executor, PgConnection, PgPool, QueryBuilder, Row};
use crate::core::duration;
use crate::error::CheeseburgerError;
use crate::model::recorder::adapter::shinhan_in;
use crate::model::recorder::config;
use crate::model::recorder::config::RecorderConfig;

const REALTIME_INSERT_QUERY: &str = r#"
INSERT INTO realtime_category_history (tr_timestamp, category_id, value)
"#;

const LOOKUP_PREVIOUS_BALANCE: &str = r#"
SELECT value::INT4 FROM bank_balance WHERE category_id = $1 AND currency_id = 1;
"#;

pub async fn start_recorder_service(pool: Arc<PgPool>)
    -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let config = config::load().await?;
    let duration_to_finish = duration::get_duration(config.close_time);

    let realtime_config = config.clone();
    let realtime_pool = pool.clone();

    let handle = tokio::spawn(async move {
        let res = record_loop(&realtime_config, realtime_pool).await;
        if let Err(e) = res {
            tracing::error!("Recorder service error: {}", e);
        }
    });

    let daily_config = config;
    let daily_pool = pool;

    tokio::spawn(async move {
        tokio::select! {
            _ = handle => (),
            _ = tokio::time::sleep(duration_to_finish) => ()
        }

        record_daily_value(&daily_config, daily_pool).await.unwrap_or_else(|e| {
            tracing::warn!("Failed to record daily value: {}", e);
        });
        tracing::info!("Recorder service finished");
    });

    Ok(())
}

async fn record_daily_value(config: &RecorderConfig, pool: Arc<PgPool>)
    -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let mut conn = pool.acquire().await?;

    let value = match shinhan_in::get_value(&config.account).await {
        Ok(value) => value,
        Err(e) => {
            tracing::error!("Failed to get value: {}", e);
            return Ok(());
        }
    };

    let previous_value = get_previous_value(&mut conn, config.category_id).await?;
    let diff = value - previous_value;
    tracing::info!("Daily value: {}", value);

    if let Err(e) = insert_daily_value(
            &mut conn,
            config.category_id,
            config.account_id,
            config.asset_id,
            diff
        ).await
    {
        tracing::warn!("Failed to record daily value: {}", e);
    }

    if let Err(e) = conn.close().await {
        tracing::warn!("Failed to close connection: {}", e);
    }

    Ok(())
}

async fn get_previous_value(
    conn: &mut PgConnection,
    category_id: i32
) -> Result<i32, Box<dyn std::error::Error + Sync + Send>>
{
    let res = conn
        .fetch_all(sqlx::query(LOOKUP_PREVIOUS_BALANCE).bind(category_id))
        .await?
        .into_iter()
        .next()
        .ok_or(CheeseburgerError::NotFoundError)?
        .try_get::<i32, _>(0)?;

    Ok(res)
}

async fn insert_daily_value(
    conn: &mut PgConnection,
    category_id: i32,
    account_id: i32,
    asset_id: i32,
    value: i32
) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let query_str = format!(
        "CALL insert_transaction(CURRENT_DATE, {}, {}, {}, {}, {}, {}, {});",
        category_id, account_id, asset_id, 5, 1, value, 0
    );

    let query = sqlx::query(&query_str);

    conn.execute(query).await?;
    Ok(())
}

async fn record_loop(config: &RecorderConfig, pool: Arc<PgPool>)
    -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    loop {
        let mut conn = pool.acquire().await?;

        let value = match shinhan_in::get_value(&config.account).await {
            Ok(value) => value,
            Err(e) => {
                tracing::warn!("Failed to get value: {}", e);
                continue;
            }
        };

        if let Err(e) =
            record_realtime_value(&mut conn, config.category_id, value).await
        {
            tracing::warn!("Failed to record realtime value: {}", e);
            continue;
        }

        if let Err(e) = conn.close().await {
            tracing::warn!("Failed to close connection: {}", e);
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn record_realtime_value(
    conn: &mut PgConnection,
    category_id: i32,
    value: i32
) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let mut query = QueryBuilder::new(REALTIME_INSERT_QUERY);

    query.push(format!(
        "VALUES(CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Seoul', {}, {});",
        category_id,
        value
    ));

    conn.execute(query.build()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use sqlx::Postgres;
    use super::*;
    use std::sync::Arc;
    use crate::core::database::get_pool;

    #[tokio::test]
    async fn test_start_recorder_service()
        -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {

        let pool = get_pool::<Postgres>().await?;
        start_recorder_service(Arc::new(pool)).await?;

        tokio::time::sleep(Duration::from_secs(100)).await;

        Ok(())
    }

    #[tokio::test]
    async fn test_record_daily_value()
        -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {
        env::set_var("RUST_LOG", "DEBUG");

        let pool = get_pool::<Postgres>().await?;
        let config = config::load().await?;

        record_daily_value(&config, Arc::new(pool)).await?;

        Ok(())
    }
}