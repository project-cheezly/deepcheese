use sqlx::{Error, Executor, PgPool};
use sqlx::postgres::PgQueryResult;

pub async fn update_category_history(pool: &PgPool)
    -> Result<PgQueryResult, Error>
{
    let mut conn = pool.acquire().await?;

    let query = include_str!("update_category_history_query.sql");

    conn.execute(query).await
}