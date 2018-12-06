use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

use super::Context;
use crate::schema::*;
use crate::models::*;

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field new_root_series(&executor, code: String) -> FieldResult<Series> {
        let conn = executor.context().pool.get()?;
        let series = Series {
            code,
            ..Series::new()
        };
        diesel::insert_into(series::table)
            .values(&series)
            .execute(&conn)?;
        Ok(series)
    }

    field new_child_series(
        &executor,
        parent_series_id: Uuid,
        code: String,
        order_in_series: i32,
    ) -> FieldResult<Series> {
        let conn = executor.context().pool.get()?;
        let series = Series {
            parent_series_id: Some(parent_series_id),
            code,
            order_in_series: Some(order_in_series),
            ..Series::new()
        };
        diesel::insert_into(series::table)
            .values(&series)
            .execute(&conn)?;
        Ok(series)
    }

    field new_book(
        &executor,
        series_id: Uuid,
        order_in_series: i32,
    ) -> FieldResult<Book> {
        let conn = executor.context().pool.get()?;
        let book = Book::new(series_id, order_in_series);
        diesel::insert_into(book::table)
            .values(&book)
            .execute(&conn)?;
        Ok(book)
    }
});
