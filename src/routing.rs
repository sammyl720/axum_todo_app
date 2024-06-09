use axum::{routing::get, Router};

pub fn init_router() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> &'static str {
    "hello world!"
}
