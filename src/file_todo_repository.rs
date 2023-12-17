use std::error::Error;
use std::fs;
use std::path::Path;
use crate::todo::{self, Todo, TodoRepository};

#[derive(Debug)]
pub struct FileTodoRepository;

impl TodoRepository for FileTodoRepository {
    fn get_todo(&self, id: usize) -> Result<Todo, Box<dyn Error>> {
        let all_todos = todo::get_todos(self, None)?;

        all_todos
            .iter()
            .find(|&todo| todo.id == id)
            .map(|todo| todo.clone())
            .ok_or_else(|| Box::from("Todo not found"))
    }

    fn get_todo_by_name(&self, name: String) -> Result<Todo, Box<dyn Error>> {
        let all_todos = todo::get_todos(self, None)?;

        all_todos
            .iter()
            .find(|&todo| todo.name == name)
            .map(|todo| todo.clone())
            .ok_or_else(|| Box::from("Todo not found"))
    }

    fn get_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
        let file_path = "nudgelyst.yaml";
        if Path::new(file_path).exists() {
            let existing_content = fs::read_to_string(file_path)?;
            let mut existing_tasks: Vec<Todo> = serde_yaml::from_str(&existing_content)?;
            if let Some(limit) = limit {
                existing_tasks = existing_tasks.iter().cloned().take(limit).collect();
            }

            return Ok(existing_tasks);
        }

        Ok(Vec::new())
    }

    fn get_completed_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
        let todos = todo::get_todos(self, None)?;

        let completed_todos: Vec<Todo> = if let Some(limit) = limit {
            todos.iter()
                .filter(|&todo| todo.completed)
                .take(limit)
                .cloned()
                .collect()
        } else {
            todos.iter()
                .filter(|&todo| todo.completed)
                .cloned()
                .collect()
        };

        Ok(completed_todos)
    }

    fn get_incomplete_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
        let todos = todo::get_todos(self, None)?;

        let incomplete_todos: Vec<Todo> = if let Some(limit) = limit {
            todos.iter()
                .filter(|&todo| !todo.completed)
                .take(limit)
                .cloned()
                .collect()
        } else {
            todos.iter()
                .filter(|&todo| !todo.completed)
                .cloned()
                .collect()
        };

        Ok(incomplete_todos)
    }

    fn create_todo(&self, mut todo: Todo) -> Result<(), Box<dyn Error>> {
        let mut existing_todos = todo::get_todos(self, None)?;

        // generate file id
        if todo.id == 0 {
            todo.id = existing_todos.len() + 1;
        }

        existing_todos.push(todo);

        Ok(write_to_file(existing_todos)?)
    }

    fn create_todos(&self, mut todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
        let mut existing_todos = todo::get_todos(self, None)?;

        // Calculate ids for todos with id 0
        let mut last_id = existing_todos.last().map(|todo| todo.id).unwrap_or(0);

        for todo in &mut todos {
            if todo.id == 0 {
                todo.id = last_id + 1;
                last_id += 1;
            }
        }

        existing_todos.extend(todos);

        Ok(write_to_file(existing_todos)?)
    }

    fn update_todo(&self, todo: Todo) -> Result<(), Box<dyn Error>> {
        let mut existing_todos = todo::get_todos(self, None)?;

        if let Some(existing_todo) = existing_todos.iter_mut().find(|t| t.id == todo.id) {
            *existing_todo = todo.clone();
        }

        Ok(write_to_file(existing_todos)?)
    }

    fn update_todos(&self, todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
        let mut existing_todos = todo::get_todos(self, None)?;

        for new_todo in &todos {
            if let Some(existing_todo) = existing_todos.iter_mut().find(|t| t.id == new_todo.id) {
                *existing_todo = new_todo.clone();
            }
        }

        Ok(write_to_file(existing_todos)?)
    }

    fn delete_todo(&self, todo: Todo) -> Result<(), Box<dyn Error>> {
        let todos = todo::get_todos(self, None)?;
        let new_todos: Vec<Todo> = todos
            .iter()
            .filter(|&todo_in_store| todo_in_store.id != todo.id)
            .cloned()
            .collect();

        // write the remaining to file
        write_to_file(new_todos)
    }

    fn delete_todos(&self, todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
        let mut existing_todos = todo::get_todos(self, None)?;

        // Extract the ids of todos to be deleted
        let todos_to_delete_ids: Vec<usize> = todos.iter().map(|todo| todo.id).collect();

        // Filter out todos with matching ids
        existing_todos.retain(|todo| !todos_to_delete_ids.contains(&todo.id));

        // write todos in storage
        write_to_file(existing_todos)?;

        Ok(())
    }
}

fn write_to_file(todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
    let file_path = "nudgelyst.yaml";
    let updated_content = serde_yaml::to_string(&todos)?;
    fs::write(file_path, updated_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    // Mock implementation for FileTodoRepository for testing purposes
    struct MockFileTodoRepository;

    // Define a constant array of string literals
    const MOCK_TODO_NAMES: [&str; 3] = ["Task 1", "Task 2", "Task 3"];

    fn get_mock_todos() -> Vec<Todo> {
        // Convert string literals to String
        let todos: Vec<Todo> = MOCK_TODO_NAMES
            .iter()
            .enumerate()
            .map(|(id, name)| Todo {
                id: id + 1,
                name: name.to_string(),
                completed: id % 2 == 1, // Every other task is completed for variety
            })
            .collect();

        todos
    }

    impl TodoRepository for MockFileTodoRepository {
        fn get_todo(&self, id: usize) -> Result<Todo, Box<dyn Error>> {
            // Simulate existing todos in the repository
            let existing_todos = get_mock_todos();

            // Find the todo with the specified ID
            if let Some(todo) = existing_todos.iter().find(|&t| t.id == id) {
                Ok(todo.clone())
            } else {
                Err(Box::from("Todo not found"))
            }
        }

        fn get_todo_by_name(&self, name: String) -> Result<Todo, Box<dyn Error>> {
            // Simulate existing todos in the repository
            let existing_todos = get_mock_todos();

            // Find the todo with the specified name
            if let Some(todo) = existing_todos.iter().find(|&t| t.name == name) {
                Ok(todo.clone())
            } else {
                Err(Box::from("Todo not found"))
            }
        }

        fn get_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
            // Simulate existing todos in the repository
            let existing_todos = get_mock_todos();

            // Apply the limit if provided
            let filtered_todos = match limit {
                Some(l) => existing_todos.iter().cloned().take(l).collect(),
                None => existing_todos,
            };

            Ok(filtered_todos)
        }

        fn get_completed_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
            // Simulate existing todos in the repository
            let existing_todos: Vec<Todo> = get_mock_todos()
                .iter()
                .cloned()
                .filter(|todo| todo.completed)
                .collect();

            // Apply the limit if provided
            let filtered_todos = match limit {
                Some(l) => existing_todos.iter().cloned().take(l).collect(),
                None => existing_todos,
            };

            Ok(filtered_todos)
        }

        fn get_incomplete_todos(&self, limit: Option<usize>) -> Result<Vec<Todo>, Box<dyn Error>> {
            // Simulate existing todos in the repository
            let existing_todos: Vec<Todo> = get_mock_todos()
                .iter()
                .cloned()
                .filter(|todo| !todo.completed)
                .collect();

            // Apply the limit if provided
            let filtered_todos = match limit {
                Some(l) => existing_todos.iter().cloned().take(l).collect(),
                None => existing_todos,
            };

            Ok(filtered_todos)
        }

        fn create_todo(&self, _todo: Todo) -> Result<(), Box<dyn Error>> {
            todo!()
        }

        fn create_todos(&self, _todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
            todo!()
        }

        fn update_todo(&self, _todo: Todo) -> Result<(), Box<dyn Error>> {
            todo!()
        }

        fn update_todos(&self, _todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
            todo!()
        }

        fn delete_todo(&self, _todo: Todo) -> Result<(), Box<dyn Error>> {
            todo!()
        }

        fn delete_todos(&self, _todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
            todo!()
        }
    }

    #[test]
    fn test_get_todo_success() {
        // Arrange
        let repo = MockFileTodoRepository;
        let todo_id = 1;

        // Act
        let result = repo.get_todo(todo_id);

        // Assert
        match result {
            Ok(todo) => {
                assert_eq!(todo.id, todo_id);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_get_todo_not_found() {
        // Arrange
        let repo = MockFileTodoRepository;
        let todo_id = 999; // Some non-existent ID

        // Act
        let result = repo.get_todo(todo_id);

        // Assert
        match result {
            Ok(_) => {
                panic!("Expected error, but got Ok");
            }
            Err(err) => {
                assert_eq!(err.to_string(), "Todo not found".to_string());
            }
        }
    }

    #[test]
    fn test_get_todo_by_name_success() {
        // Arrange
        let repo = MockFileTodoRepository;
        let todo_name = "Task 1".to_string();

        // Act
        let result = repo.get_todo_by_name(todo_name.clone());

        // Assert
        match result {
            Ok(todo) => {
                assert_eq!(todo.name, todo_name);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_get_todo_by_name_not_found() {
        // Arrange
        let repo = MockFileTodoRepository;
        let todo_name = "".to_string();

        // Act
        let result = repo.get_todo_by_name(todo_name.clone());

        // Assert
        match result {
            Ok(_) => {
                panic!("Expected error, but got Ok");
            }
            Err(err) => {
                assert_eq!(err.to_string(), "Todo not found".to_string());
            }
        }
    }

    #[test]
    fn test_get_todos_with_limit() {
        // Arrange
        let repo = MockFileTodoRepository;
        let limit = Some(2);

        // Act
        let result = repo.get_todos(limit);

        // Assert
        match result {
            Ok(todos) => {
                assert_eq!(todos.len(), 2);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_get_todos_no_limit() {
        // Arrange
        let repo = MockFileTodoRepository;

        // Act
        let result = repo.get_todos(None);

        let existing_todos = get_mock_todos();

        // Assert
        match result {
            Ok(todos) => {
                assert_eq!(todos, existing_todos);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_get_completed_todos_with_limit() {
        // Arrange
        let repo = MockFileTodoRepository;
        let limit = Some(2);

        // Act
        let result = repo.get_completed_todos(limit);

        // Assert
        match result {
            Ok(todos) => {
                assert_eq!(todos.len(), 1);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_get_completed_todos_no_limit() {
        // Arrange
        let repo = MockFileTodoRepository;

        // Act
        let result = repo.get_completed_todos(None);

        let existing_completed_todos: Vec<Todo> = get_mock_todos()
            .iter()
            .cloned()
            .filter(|todo| todo.completed)
            .collect();

        // Assert
        match result {
            Ok(todos) => {
                assert_eq!(todos, existing_completed_todos);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_get_incomplete_todos_with_limit() {
        // Arrange
        let repo = MockFileTodoRepository;
        let limit = Some(2);

        // Act
        let result = repo.get_incomplete_todos(limit);

        // Assert
        match result {
            Ok(todos) => {
                assert_eq!(todos.len(), 2);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_get_incomplete_todos_no_limit() {
        // Arrange
        let repo = MockFileTodoRepository;

        // Act
        let result = repo.get_incomplete_todos(None);

        let existing_incomplete_todos: Vec<Todo> = get_mock_todos()
            .iter()
            .cloned()
            .filter(|todo| !todo.completed)
            .collect();

        // Assert
        match result {
            Ok(todos) => {
                assert_eq!(todos, existing_incomplete_todos);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }
}