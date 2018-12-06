mod context;
mod mutation;
mod query;

pub use self::context::Context;
pub use self::mutation::Mutation;
pub use self::query::Query;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn new_schema() -> Schema {
    Schema::new(Query, Mutation)
}
