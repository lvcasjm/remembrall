use chrono::{NaiveDateTime, NaiveTime};
use inquire::{DateSelect, Select, Text};
use sqlx::{Pool, Sqlite};

use crate::media::Media;

pub fn prompt() -> anyhow::Result<Option<Media>, anyhow::Error> {
    println!(
        "Let's start by logging something you finished watching, reading, or playing recently."
    );

    let media_types = vec!["Movie", "Game", "Series", "Book"];

    let media_type = Select::new("What type of Media have you completed?", media_types)
        .prompt()
        .unwrap();

    let title = Text::new("What is the title of the media you have finished?")
        .prompt()
        .unwrap();

    let description = Text::new("(Optional) Add a description of the media you have finished?")
        .prompt()
        .unwrap();

    let date = DateSelect::new("When did you finish the media?")
        .prompt()
        .unwrap();

    let as_date_time = NaiveDateTime::new(date, NaiveTime::default());

    let media = Media {
        id: None,
        title,
        media_type: media_type.parse().unwrap(),
        description,
        completed_at: as_date_time,
    };

    println!("Title: {}", media.title);
    println!("Type: {}", media.media_type);
    println!("Description: {}", media.description);
    println!("Completed Date: {}", media.completed_at);

    let confirm_options = vec!["Save", "Cancel"];
    let confirmation = Select::new("Would you like to save this media?", confirm_options).prompt();

    match confirmation {
        Ok("Save") => Ok(Some(media)),
        _ => Ok(None),
    }
}

pub async fn save(media: &Media, pool: Pool<Sqlite>) -> anyhow::Result<bool> {
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
