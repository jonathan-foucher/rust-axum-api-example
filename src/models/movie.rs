use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub release_date: NaiveDate
}
