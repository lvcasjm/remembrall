use crate::media::Media;
use sqlx::MySqlPool;
use std::env;

pub(crate) async fn save(media: &Media) -> anyhow::Result<bool> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

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
