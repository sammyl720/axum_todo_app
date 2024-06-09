use axum::{
    response::Json,
    routing::{get, post},
    Extension, Router,
};

use sqlx::SqlitePool;

use crate::{
    app_err::AppError,
    database::{create_todo, get_all_todos, init_db},
    todos::{CreateTodo, NewTodo, Todo},
};

#[derive(Clone)]
pub struct AppState {
    db: SqlitePool,
}

pub async fn init_router() -> Result<Router, AppError> {
    let pool = init_db().await?;

    let state = AppState { db: pool };

    Ok(Router::new()
        .route("/", get(get_todos))
        .route("/", post(add_todo))
        .layer(Extension(state)))
}

async fn get_todos(Extension(state): Extension<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    get_all_todos(&state.db)
        .await
        .map(|todos| Json(todos))
        .map_err(|err: anyhow::Error| AppError(err))
}

async fn add_todo(
    Extension(state): Extension<AppState>,
    new_todo: Json<CreateTodo>,
) -> Result<Json<NewTodo>, AppError> {
    let result = create_todo(&state.db, new_todo.0).await;

    result
        .map(|id| Json(NewTodo { id }))
        .map_err(|err: anyhow::Error| AppError(err))
}
