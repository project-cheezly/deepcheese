use std::env;
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub async fn connect_to_database() -> Result<Pool<Postgres>, Error> {
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_port = env::var("DB_PORT")
        .expect("DB_PORT must be set")
        .parse::<u16>()
        .expect("DB_PORT must be a number");
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());

    let connect_options = PgConnectOptions::new()
        .host(&db_host)
        .username(&db_user)
        .password(&db_password)
        .port(db_port)
        .database(&db_name);

    PgPoolOptions::new()
        .max_connections(3)
        .connect_with(connect_options).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect_to_database() {
        let pool = connect_to_database().await;
        assert!(pool.is_ok());
    }
}