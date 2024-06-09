use dotenv::dotenv;
use rest::init_router;

mod app_err;
mod database;
mod rest;
mod todos;

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    dotenv().ok();
    // init router app
    let app = init_router().await.unwrap();

    // create tcp listener
    let listener = tokio::net::TcpListener::bind(HOST).await.unwrap();

    // create server with listener and app
    axum::serve(listener, app).await.unwrap();
}
