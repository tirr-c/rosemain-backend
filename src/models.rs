use chrono::{Date, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Series {
    pub id: Uuid,
    pub parent_series_id: Option<Uuid>,
    pub code: String,
    pub order_in_series: i32,
}

#[derive(Queryable)]
pub struct Book {
    pub id: Uuid,
    pub series_id: Uuid,
    pub order_in_series: i32,
}

#[derive(Queryable)]
pub struct SeriesInfo {
    pub id: Uuid,
    pub series_id: Uuid,
    pub lang: String,
    pub name: String,
}

#[derive(Queryable)]
pub struct BookInfo {
    pub id: Uuid,
    pub book_id: Uuid,
    pub lang: String,
    pub title: String,
    pub isbn: String,
    pub published_at: Date<Utc>,
}

#[derive(Queryable)]
pub struct Person {
    pub id: Uuid,
    pub lang: String,
    pub name: String,
}
