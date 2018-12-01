use chrono::NaiveDate;
use uuid::prelude::*;

use crate::schema::*;

#[derive(Identifiable, Queryable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Series, foreign_key = "parent_series_id")]
#[table_name = "series"]
pub struct Series {
    pub id: Uuid,
    pub parent_series_id: Option<Uuid>,
    pub code: String,
    pub order_in_series: Option<i32>,
}

impl Series {
    pub fn new() -> Self {
        Series {
            id: Uuid::new_v4(),
            parent_series_id: None,
            code: String::new(),
            order_in_series: None,
        }
    }

    pub fn build_child_series(&self) -> Self {
        Series {
            id: Uuid::new_v4(),
            parent_series_id: Some(self.id),
            code: String::new(),
            order_in_series: None,
        }
    }

    pub fn build_child_book(&self) -> Book {
        Book {
            id: Uuid::new_v4(),
            series_id: self.id,
            order_in_series: 0,
        }
    }
}

#[derive(Identifiable, Queryable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Series)]
#[table_name = "book"]
pub struct Book {
    pub id: Uuid,
    pub series_id: Uuid,
    pub order_in_series: i32,
}

#[derive(Identifiable, Queryable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Series)]
#[table_name = "series_info"]
pub struct SeriesInfo {
    pub id: Uuid,
    pub series_id: Uuid,
    pub lang: String,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Book)]
#[table_name = "book_info"]
pub struct BookInfo {
    pub id: Uuid,
    pub book_id: Uuid,
    pub lang: String,
    pub title: String,
    pub isbn: String,
    pub published_at: NaiveDate,
}

#[derive(Identifiable, Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "person"]
pub struct Person {
    pub id: Uuid,
    pub lang: String,
    pub name: String,
}
