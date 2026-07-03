use chrono::NaiveDateTime;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

use crate::{database, media::Media};

pub async fn list() -> anyhow::Result<Vec<Media>> {
    let pool = database::connection().await.unwrap();

    let statement = format!(
        "
        SELECT id, title, description, media_type, completed_at FROM media;
        ",
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

pub async fn show_table() {
    let mut table = Table::new();

    let list = list().await.unwrap();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Title").add_attribute(Attribute::Bold),
            Cell::new("Description").add_attribute(Attribute::Bold),
            Cell::new("Type").add_attribute(Attribute::Bold),
            Cell::new("Completed").add_attribute(Attribute::Bold),
        ]);

    for row in list {
        table.add_row(vec![
            row.title,
            row.description,
            row.media_type,
            row.completed_at.format("%d/%m/%Y").to_string(),
        ]);
    }

    println!("{table}");
}
