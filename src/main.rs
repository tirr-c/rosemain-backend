#![feature(async_await, futures_api)]

use http::StatusCode;
use rosemain::{connection, graphql::{self, Context}};
use tide::{body, App, AppData, Response};

async fn graphql(
    ctx: AppData<Context>,
    query: body::Json<juniper::http::GraphQLRequest>,
) -> Result<Response, StatusCode> {
    let response = query.execute(&graphql::new_schema(), &ctx);
    let body_vec = serde_json::to_vec(&response).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    http::Response::builder()
        .status(if response.is_ok() {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        })
        .header("Content-Type", "application/json")
        .body(body::Body::from(body_vec))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn main() {
    let pool = connection::establish_connection();
    let ctx = Context::new(pool);
    let mut app = App::new(ctx);

    app.at("/graphql").post(graphql);

    let address = "127.0.0.1:8000".to_owned();
    app.serve(address);
}
