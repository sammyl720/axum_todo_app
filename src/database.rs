use anyhow::{Context, Ok, Result};

use std::env;

use sqlx::{Pool, Row, Sqlite, SqlitePool};

use crate::todos::{CreateTodo, Todo};

pub async fn init_db() -> Result<Pool<Sqlite>> {
    SqlitePool::connect(&env::var("DATABASE_URL")?)
        .await
        .with_context(|| String::from("Failed to connect to db"))
}

pub async fn create_todo(conn: &SqlitePool, new_todo: CreateTodo) -> Result<i32, anyhow::Error> {
    Ok(
        sqlx::query("INSERT into todos (description) VALUES ($1) RETURNING id")
            .bind(new_todo.description)
            .fetch_one(conn)
            .await
            .with_context(|| "could not create todo")?
            .get(0),
    )
}

pub async fn get_all_todos(conn: &SqlitePool) -> Result<Vec<Todo>> {
    sqlx::query_as!(
        Todo,
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(conn)
    .await
    .with_context(|| "Could not get all todos from database")
}

pub async fn update_todo(conn: &SqlitePool, todo: Todo) -> Result<i64> {
    sqlx::query("UPDATE todos SET description=$1, done=$2 WHERE id=$3")
        .bind(&todo.description)
        .bind(&todo.done)
        .bind(&todo.id)
        .execute(conn)
        .await
        .with_context(|| "Could not update todo in database")
        .map(|_| todo.id)
}
