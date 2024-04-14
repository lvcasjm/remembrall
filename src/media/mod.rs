use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Media {
    pub id: Option<i32>,
    pub title: String,
    pub media_type: String,
    pub description: String,
    pub completed_at: NaiveDateTime,
}
