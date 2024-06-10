use std::{env, net::SocketAddr};

use axum::{routing::get, Router};
use dotenv::dotenv;
use rest::todo_service;
use tower_http::services::ServeDir;

mod app_err;
mod database;
mod rest;
mod todos;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // init router app
    let app = Router::new()
        .route("/health_check", get(health_check))
        .nest_service("/", ServeDir::new("public"))
        .nest_service("/api/todos", todo_service().await.unwrap());

    let port: u16 = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    // create tcp listener
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on {}", addr);
    // create server with listener and app
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
