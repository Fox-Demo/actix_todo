use crate::models::todo::Todo;
use chrono::prelude::*;
use std::fmt::Error;
use std::sync::{Arc, Mutex};

pub struct Database {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        //Ok(todo)
        let mut todos = self.todos.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let todo = Todo {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };

        todos.push(todo.clone());
        Ok(todo)
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }

    pub fn get_todo_by_id(&self, id: &str) -> Option<Todo> {
        let todos = self.todos.lock().unwrap();
        let todo = todos.iter().find(|t| t.id.as_deref() == Some(id));
        todo.cloned()
    }
}
