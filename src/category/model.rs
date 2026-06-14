use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Category {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}
