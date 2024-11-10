use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use diesel::prelude::{Insertable, Queryable, Selectable};

#[derive(Insertable, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::movie)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub release_date: NaiveDate
}
