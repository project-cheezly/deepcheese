use sqlx::Pool;

mod config;

pub async fn get_pool<T>()
    -> Result<Pool<T>, Box<dyn std::error::Error + Sync + Send>>
    where T: sqlx::Database
{
    let config = config::load().await?;
    Ok(Pool::connect(&config.to_string()).await?)
}
