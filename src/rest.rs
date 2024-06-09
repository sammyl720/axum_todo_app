use axum::{
    extract::Path,
    response::Json,
    routing::{delete, get, post, put},
    Extension, Router,
};

use sqlx::SqlitePool;

use crate::{
    app_err::AppError,
    database::{create_todo, delete_todo, get_all_todos, get_single_todo, init_db, update_todo},
    todos::{CreateTodo, Entity, NewTodo, Todo},
};

#[derive(Clone)]
pub struct AppState {
    db: SqlitePool,
}

pub async fn todo_service() -> Result<Router, AppError> {
    let pool = init_db().await?;

    let state = AppState { db: pool };

    Ok(Router::new()
        .route("/", get(get_todos))
        .route("/:id", get(get_todo))
        .route("/", post(add_todo))
        .route("/edit", put(edit_todo))
        .route("/delete/:id", delete(remove_todo))
        .layer(Extension(state)))
}

async fn get_todos(Extension(state): Extension<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    get_all_todos(&state.db)
        .await
        .map(|todos| Json(todos))
        .map_err(|err: anyhow::Error| AppError(err))
}

async fn get_todo(
    Extension(state): Extension<AppState>,
    Path(todo_id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    get_single_todo(&state.db, todo_id)
        .await
        .map(|todo| Json(todo))
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

async fn edit_todo(
    Extension(state): Extension<AppState>,
    todo: Json<Todo>,
) -> Result<Entity, AppError> {
    update_todo(&state.db, todo.0)
        .await
        .map(Entity::new)
        .map_err(|err| AppError(err))
}

async fn remove_todo(
    Extension(state): Extension<AppState>,
    Path(todo_id): Path<i64>,
) -> Result<Entity, AppError> {
    delete_todo(&state.db, todo_id)
        .await
        .map(Entity::new)
        .map_err(|err| AppError(err))
}
