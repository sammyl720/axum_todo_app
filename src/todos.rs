use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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
