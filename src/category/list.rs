use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::Row;
use sqlx::{sqlite::SqliteRow, SqlitePool};

use crate::{category::Category, config::RemembrallConfig};

pub async fn list() -> anyhow::Result<Vec<Category>> {
    dotenv().ok();

    let config: RemembrallConfig = confy::load("remembrall", None)?;

    let pool = SqlitePool::connect(&config.connection_url).await?;

    let statement = format!(
        "
        SELECT id, title, description, created_at FROM category;
        ",
    );

    let query = sqlx::query(&statement)
        .map(|row: SqliteRow| Category {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            created_at: row.get::<NaiveDateTime, &str>("created_at"),
        })
        .fetch_all(&pool)
        .await?;

    Ok(query)
}
