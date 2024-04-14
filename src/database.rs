use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::mysql::MySqlRow;
use sqlx::MySqlPool;
use sqlx::Row;

use crate::media::Media;
use crate::RemembrallConfig;

pub(crate) async fn list() -> anyhow::Result<Vec<Media>> {
    dotenv().ok();

    let config: RemembrallConfig = confy::load("remembrall", None)?;

    let pool = MySqlPool::connect(&config.connection_url).await?;

    let query = sqlx::query("SELECT id, title, description, media_type, completed_at FROM media")
        .map(|row: MySqlRow| Media {
            id: row.get("id"),
            title: row.get("title"),
            media_type: row.get("media_type"),
            description: row.get("description"),
            completed_at: row.get::<NaiveDateTime, &str>("completed_at"),
        })
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(query)
}

pub(crate) async fn save(media: &Media) -> anyhow::Result<bool> {
    dotenv().ok();

    let config: RemembrallConfig = confy::load("remembrall", None)?;

    let pool = MySqlPool::connect(&config.connection_url).await?;

    println!("Please wait, saving in progress!");

    sqlx::query!(
        r"
            CREATE TABLE IF NOT EXISTS media (
                id INT NOT NULL AUTO_INCREMENT,
                title VARCHAR(255) NOT NULL,
                description VARCHAR(255) NOT NULL,
                media_type VARCHAR(255) NOT NULL,
                completed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (id)
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
