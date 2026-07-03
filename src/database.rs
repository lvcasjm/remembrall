use crate::config::RemembrallConfig;
use dotenv::dotenv;
use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::SqlitePool;

pub async fn connection() -> anyhow::Result<Pool<Sqlite>> {
    dotenv().ok();

    let config: RemembrallConfig = confy::load("remembrall", None)?;

    let pool = SqlitePool::connect(&config.connection_url).await?;

    Ok(pool)
}
