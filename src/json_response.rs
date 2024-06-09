use std::vec;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

async fn messager() -> ApiResponse {
    let messages = vec![Message {
        message: "hello there".to_string(),
    }];
    ApiResponse::JsonData(messages)
}

pub async fn init_router() -> Router {
    Router::new().route("/", get(messager))
}
