use axum::Router;
use dotenv::dotenv;
use rest::todo_service;
use tower_http::services::ServeDir;

mod app_err;
mod database;
mod rest;
mod todos;

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    dotenv().ok();
    // init router app
    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .nest_service("/api/todos", todo_service().await.unwrap());

    // create tcp listener
    let listener = tokio::net::TcpListener::bind(HOST).await.unwrap();

    println!("Starting server on {}", HOST);
    // create server with listener and app
    axum::serve(listener, app).await.unwrap();
}
