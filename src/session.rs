use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, PooledConnection},
};

use crate::{
    connection::ConnectionPool,
    models::*,
    schema,
};

pub struct Session {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl Session {
    pub fn new(pool: &ConnectionPool) -> Result<Session, r2d2::PoolError> {
        let conn = pool.get()?;
        Ok(Session { conn })
    }

    pub fn fetch_series(&self, code: &str) -> Option<Series> {
        use self::schema::series::{self, columns};

        series::table
            .filter(columns::code.eq(code))
            .select(series::all_columns)
            .first(&self.conn)
            .optional()
            .unwrap()
    }

    pub fn fetch_children_series(&self, parent: &Series) -> Vec<Series> {
        use self::schema::series;

        Series::belonging_to(parent)
            .select(series::all_columns)
            .load(&self.conn)
            .unwrap()
    }

    pub fn save_series(&self, series: &Series) {
        use self::schema::series::{self, columns};

        diesel::insert_into(series::table)
            .values(series)
            .on_conflict(columns::id)
            .do_update()
            .set(series)
            .execute(&self.conn)
            .unwrap();
    }

    pub fn fetch_children_books(&self, parent: &Series) -> Vec<Book> {
        use self::schema::book;

        Book::belonging_to(parent)
            .select(book::all_columns)
            .load(&self.conn)
            .unwrap()
    }

    pub fn save_book(&self, book: &Book) {
        use self::schema::book::{self, columns};

        diesel::insert_into(book::table)
            .values(book)
            .on_conflict(columns::id)
            .do_update()
            .set(book)
            .execute(&self.conn)
            .unwrap();
    }
}
