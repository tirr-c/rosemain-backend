use chrono::NaiveDate;
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::prelude::*;

use crate::graphql::Context;
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

impl Default for Series {
    fn default() -> Self {
        Series::new()
    }
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
}

graphql_object!(Series: Context |&self| {
    field id() -> Uuid {
        self.id
    }

    field code() -> &str {
        &self.code
    }

    field order_in_series() -> Option<i32> {
        self.order_in_series
    }

    field children_series(&executor) -> FieldResult<Vec<Series>> {
        let conn = executor.context().pool.get()?;
        let series = Series::belonging_to(self)
            .select(series::all_columns)
            .load(&conn)?;
        Ok(series)
    }

    field series(&executor, order_in_series: i32) -> FieldResult<Series> {
        let conn = executor.context().pool.get()?;
        let result = series::table
            .filter(series::columns::parent_series_id.eq(Some(self.id)))
            .filter(series::columns::order_in_series.eq(Some(order_in_series)))
            .select(series::all_columns)
            .first(&conn)?;
        Ok(result)
    }

    field children_books(&executor) -> FieldResult<Vec<Book>> {
        let conn = executor.context().pool.get()?;
        let books = Book::belonging_to(self)
            .select(book::all_columns)
            .load(&conn)?;
        Ok(books)
    }

    field book(&executor, order_in_series: i32) -> FieldResult<Book> {
        let conn = executor.context().pool.get()?;
        let result = book::table
            .filter(book::columns::series_id.eq(self.id))
            .filter(book::columns::order_in_series.eq(order_in_series))
            .select(book::all_columns)
            .first(&conn)?;
        Ok(result)
    }

    field parent_series(&executor) -> FieldResult<Option<Series>> {
        let conn = executor.context().pool.get()?;
        let result = if let Some(id) = self.parent_series_id {
            Some(
                series::table
                    .find(id)
                    .first(&conn)?
            )
        } else {
            None
        };
        Ok(result)
    }
});

#[derive(Identifiable, Queryable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Series)]
#[table_name = "book"]
pub struct Book {
    pub id: Uuid,
    pub series_id: Uuid,
    pub order_in_series: i32,
}

impl Book {
    pub fn new(series_id: Uuid, order_in_series: i32) -> Self {
        Book {
            id: Uuid::new_v4(),
            series_id,
            order_in_series,
        }
    }
}

graphql_object!(Book: Context |&self| {
    field id() -> Uuid {
        self.id
    }

    field order_in_series() -> i32 {
        self.order_in_series
    }

    field parent_series(&executor) -> FieldResult<Series> {
        let conn = executor.context().pool.get()?;
        let result = series::table
            .find(self.series_id)
            .first(&conn)?;
        Ok(result)
    }
});

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
