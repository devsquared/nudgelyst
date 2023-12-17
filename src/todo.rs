use serde::{Deserialize, Serialize};
use std::error::Error;

pub trait TodoRepository {
    fn get_todo(&self, id: usize) -> Result<Todo, Box<dyn Error>>;
    fn get_todo_by_name(&self, name: String) -> Result<Todo, Box<dyn Error>>;
    fn get_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>>;
    fn get_completed_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>>;
    fn get_incomplete_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>>;
    fn create_todo(&self, todo: Todo) -> Result<(), Box<dyn Error>>;
    fn create_todos(&self, todos: Vec<Todo>) -> Result<(), Box<dyn Error>>;
    fn update_todo(&self, todo: Todo) -> Result<(), Box<dyn Error>>;
    fn update_todos(&self, todos: Vec<Todo>) -> Result<(), Box<dyn Error>>;
    fn delete_todo(&self, todo: Todo) -> Result<(), Box<dyn Error>>;
    fn delete_todos(&self, todos: Vec<Todo>) -> Result<(), Box<dyn Error>>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: usize, name: String) -> Todo {
        Todo {
            id,
            name,
            completed: false,
        }
    }
}

// pub fn get_todo(repo: &dyn TodoRepository, id: usize) -> Result<Todo, Box<dyn Error>> {
//     repo.get_todo(id)
// }
//
// pub fn get_todo_by_name(repo: &dyn TodoRepository, name: String) -> Result<Todo, Box<dyn Error>> {
//     repo.get_todo_by_name(name)
// }

pub fn get_todos(repo: &dyn TodoRepository, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
    repo.get_todos(limit)
}

// pub fn get_completed_todos(repo: &dyn TodoRepository, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
//     repo.get_completed_todos(limit)
// }
// fn get_incomplete_todos(repo: &dyn TodoRepository, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
//     repo.get_incomplete_todos(limit)
// }

// pub fn create_todo(repo: &dyn TodoRepository, todo: Todo) -> Result<(), Box<dyn Error>> {
//     repo.create_todo(todo)
// }

// pub fn create_todos(repo: &dyn TodoRepository, todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
//     repo.create_todos(todos)
// }

// pub fn update_todo(repo: &dyn TodoRepository, todo: Todo) -> Result<(), Box<dyn Error>> {
//     repo.update_todo(todo)
// }
// pub fn update_todos(repo: &dyn TodoRepository, todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
//     repo.update_todos(todos)
// }
// pub fn delete_todo(repo: &dyn TodoRepository, todo: Todo) -> Result<(), Box<dyn Error>> {
//     repo.delete_todo(todo)
// }
// pub fn delete_todos(repo: &dyn TodoRepository, todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
//     repo.delete_todos(todos)
// }

