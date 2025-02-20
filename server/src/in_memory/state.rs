use std::sync::{Arc, Mutex, RwLock};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize)]
pub(crate) struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub todos: Arc<RwLock<Vec<Todo>>>, // Vec<Todo>,
    pub next_id: Arc<Mutex<i32>>,
}

impl AppState {
    pub fn new(todos: Vec<Todo>) -> Self {
        let max_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0);
        Self {
            todos: Arc::new(RwLock::new(todos)),
            next_id: Arc::new(Mutex::new(max_id + 1)),
        }
    }

    pub fn get_id(&self) -> i32 {
        let mut next_id = self.next_id.lock().expect("mutex poised");
        let id = *next_id;
        *next_id += 1;
        id
    }
}

pub(crate) fn load_state() -> AppState {
    let todo = vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Learn Axum".to_string(),
            completed: false,
        },
    ];

    AppState::new(todo)
}
