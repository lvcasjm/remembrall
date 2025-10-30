use crate::config::RemembrallConfig;
use crate::media::Media;
use chrono::Datelike;
use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use sqlx::SqlitePool;

pub async fn list() -> anyhow::Result<Vec<Media>> {
    dotenv().ok();

    let config: RemembrallConfig = confy::load("remembrall", None)?;

    let pool = SqlitePool::connect(&config.connection_url).await?;

    let current_date = chrono::offset::Local::now();

    let _year = current_date.year().to_string();

    // WHERE completed_at BETWEEN '{year}-01-01' AND '{year}-12-31';
    let statement = format!(
        "
        SELECT id, title, description, media_type, completed_at FROM media;
        ",
        // year = year
    );

    let query = sqlx::query(&statement)
        .map(|row: SqliteRow| Media {
            id: row.get("id"),
            title: row.get("title"),
            media_type: row.get("media_type"),
            description: row.get("description"),
            completed_at: row.get::<NaiveDateTime, &str>("completed_at"),
        })
        .fetch_all(&pool)
        .await?;

    Ok(query)
}

pub async fn save(media: &Media) -> anyhow::Result<bool> {
    dotenv().ok();

    let config: RemembrallConfig = confy::load("remembrall", None)?;

    let pool = SqlitePool::connect(&config.connection_url).await?;

    println!("Please wait, saving in progress!");

    sqlx::query!(
        r"
            CREATE TABLE IF NOT EXISTS media (
                id INTEGER PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                description VARCHAR(255) NOT NULL,
                media_type VARCHAR(255) NOT NULL,
                completed_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        ",
    )
    .execute(&pool)
    .await?;

    let rows_affected = sqlx::query!(
        r#"
            INSERT INTO media (
                title,
                media_type,
                description,
                completed_at
            )
            VALUES (
                ?,
                ?,
                ?,
                ?
            );
        "#,
        media.title,
        media.media_type,
        media.description,
        media.completed_at,
    )
    .execute(&pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}
