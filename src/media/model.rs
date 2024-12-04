use chrono::NaiveDateTime;
use reqwest::{header::CONTENT_TYPE, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Media {
    pub id: Option<i32>,
    pub title: String,
    pub media_type: String,
    pub description: String,
    pub completed_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    items: Vec<Book>,
}

impl Media {
    async fn fetch_book_metadata(media: &Self) -> anyhow::Result<(), Error> {
        println!("Fetching book metadata");

        let client = reqwest::Client::new();

        let request_url = format!(
            "https://www.googleapis.com/books/v1/volumes?q={t}",
            t = media.title
        );

        let response = client
            .get(&request_url)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?;

        let books: ApiResponse = response.json().await?;

        println!("{:?}", books);

        Ok(())
    }

    async fn fetch_game_metadata(media: &Self) -> anyhow::Result<(), Error> {
        Ok(())
    }

    async fn fetch_series_metadata(media: &Self) -> anyhow::Result<(), Error> {
        Ok(())
    }

    async fn fetch_movie_metadata(media: &Self) -> anyhow::Result<(), Error> {
        Ok(())
    }

    pub async fn fetch_metadata(media: &Self) -> anyhow::Result<(), Error> {
        match media.media_type.as_str() {
            "Book" => Self::fetch_book_metadata(media).await?,
            "Game" => Self::fetch_game_metadata(media).await?,
            "Movie" => Self::fetch_movie_metadata(media).await?,
            "Series" => Self::fetch_series_metadata(media).await?,
            _ => (),
        }

        Ok(())
    }
}
