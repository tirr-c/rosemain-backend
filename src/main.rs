#![feature(async_await, futures_api)]

use http::StatusCode;
use rosemain::{connection, graphql::{self, Context}};
use tide::{body, configuration, App, AppData, IntoResponse, Response};

async fn graphql(
    ctx: AppData<Context>,
    query: body::Json<juniper::http::GraphQLRequest>,
) -> Response {
    let response = query.execute(&graphql::new_schema(), &ctx);
    let status = if response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    body::Json(response)
        .with_status(status)
        .into_response()
}

fn main() {
    let pool = connection::establish_connection();
    let ctx = Context::new(pool);
    let mut app = App::new(ctx);

    let conf = configuration::Configuration::build()
        .port(8000)
        .finalize();
    app.config(conf);
    app.at("/graphql").post(graphql);
    app.serve();
}
