use chrono::NaiveDate;

#[derive(Debug)]
pub struct Media {
    pub title: String,
    pub media_type: String,
    pub description: String,
    pub completed_at: NaiveDate,
}
