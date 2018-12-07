use crate::connection::ConnectionPool;

#[derive(Clone)]
pub struct Context {
    pub(crate) pool: ConnectionPool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(pool: ConnectionPool) -> Self {
        Context {
            pool,
        }
    }
}
