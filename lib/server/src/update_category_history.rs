use kis::KIS;
use sqlx::{Error, Executor, PgPool, Postgres};
use sqlx::postgres::PgQueryResult;

pub async fn update_category_history(pool: &PgPool, kis: &KIS)
    -> Result<PgQueryResult, Error>
{
    let mut conn = pool.acquire().await?;

    let query = include_str!("update_category_history_query.sql");
    let tr_date = chrono::offset::Utc::now().date_naive();

    conn.execute(sqlx::query_as::<Postgres, ()>(query).bind(tr_date)).await
}