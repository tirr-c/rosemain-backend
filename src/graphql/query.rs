use diesel::prelude::*;
use juniper::FieldResult;

use super::Context;
use crate::models::*;
use crate::schema::*;

pub struct Query;

graphql_object!(Query: Context |&self| {
    field series(&executor, code: String) -> FieldResult<Series> {
        let conn = executor.context().pool.get()?;
        let result = series::table
            .filter(series::columns::code.eq(&code))
            .first(&conn)?;
        Ok(result)
    }
});
