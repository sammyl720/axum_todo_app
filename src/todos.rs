use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateTodo {
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewTodo {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub id: i64,
}

impl Entity {
    pub fn new(id: i64) -> Self {
        Entity { id }
    }
}
impl IntoResponse for Entity {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
