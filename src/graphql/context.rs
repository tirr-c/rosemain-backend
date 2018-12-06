pub struct Context {
    pub(crate) pool: crate::connection::ConnectionPool,
}

impl juniper::Context for Context {}
