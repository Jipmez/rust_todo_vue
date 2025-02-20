use axum::{
    extract::{Path, State},
    routing::{get, patch},
    Json, Router,
};

use super::{
    app_state::AppState,
    store::{Store, Todo},
};

use serde::Deserialize;

#[derive(Deserialize)]
struct NewTodo {
    pub title: String,
}

pub(crate) fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/{id}", patch(update_todo).delete(delete_todo))
}

async fn get_todos(State(store): State<Store>) -> Json<Vec<Todo>> {
    Json(store.get_todos().await.unwrap())
}

async fn create_todo(State(store): State<Store>, Json(todo): Json<NewTodo>) -> Json<Vec<Todo>> {
    let todo = store.add_todo_return_all(todo.title).await.unwrap();
    Json(todo)
}

async fn update_todo(
    State(store): State<Store>,
    Path(id): Path<i32>,
    Json(todo): Json<Todo>,
) -> Json<Vec<Todo>> {
    let todo = store
        .update_todo_return_all(id, todo.title, todo.completed)
        .await
        .unwrap();
    Json(todo)
}

async fn delete_todo(State(store): State<Store>, Path(id): Path<i32>) -> Json<Vec<Todo>> {
    let todo = store.delete_todo_return_all(id).await.unwrap();
    Json(todo)
}
