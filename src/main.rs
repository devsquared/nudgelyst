mod todo;
mod file_todo_repository;

use std::error::Error;
use clap::{Parser, Subcommand};
use colored::Colorize;
use todo::*;
use file_todo_repository::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

//TODO: we will want a 'clean' command that removes completed from the list
//TODO: add a 'togone' command to delete a todo
//TODO: add a 'wipe' command to delete all todos and start fresh!

#[derive(Subcommand, Debug)]
#[clap(rename_all = "snake_case")]
pub enum Command {
    Todo {
        name: String,
    },
    Todone {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        id: Option<usize>,
    },
    Tundo {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        id: Option<usize>,
    },
    List {
        #[arg(short, long)]
        completed: Option<bool>,
        #[arg(short, long)]
        incomplete: Option<bool>,
        #[arg(short, long)]
        limit: Option<usize>,
    },
    Togone {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        id: Option<usize>,
    },
    Clean {
    },
    Wipe {
    }
}
fn main() {
    let file_repo = FileTodoRepository;

    let args = Args::parse();

    match args.command {
        Command::Todo { name } => {
            if let Err(err) = file_repo.create_todo(Todo::new(0, name)) {
                eprintln!("error writing to nudgelyst.yaml file: Error: {}", err);
            }

            if let Err(err) = list_last_five(file_repo) {
                eprintln!("error printing last 5: Error: {}", err);
            }
        },
        Command::Todone { name, id } => match (name, id) {
            (Some(todo_name), None) => {
                if let Some(todo) = file_repo.get_todo_by_name(todo_name.clone()).ok().map(|t| t) {
                    if let Err(err) = file_repo.update_todo(Todo { id: todo.id, name: todo.name.clone(), completed: true }) {
                        eprintln!("error marking '{}' todo completed: Error: {}", todo_name, err);
                    }
                } else {
                    eprintln!("Todo with name '{}' not found.", todo_name);
                }


            },
            (None, Some(todo_id)) => {
                if let Some(todo) = file_repo.get_todo(todo_id).ok().map(|t| t) {
                    if let Err(err) = file_repo.update_todo(Todo { id: todo.id, name: todo.name.clone(), completed: true }) {
                        eprintln!("error marking '{}' todo completed: Error: {}", todo_id, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_id);
                }
            },
            (Some(_), Some(todo_id)) => {
                if let Some(todo) = file_repo.get_todo(todo_id).ok().map(|t| t) {
                    if let Err(err) = file_repo.update_todo(Todo { id: todo.id, name: todo.name.clone(), completed: true }) {
                        eprintln!("error marking '{}' todo completed: Error: {}", todo_id, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_id);
                }
            },
            (None, None) => panic!("Need either a name or id argument to complete a task"),
        },
        Command::Tundo { name, id } => match (name, id) {
            (Some(todo_name), None) => {
                if let Some(todo) = file_repo.get_todo_by_name(todo_name.clone()).ok().map(|t| t) {
                    if let Err(err) = file_repo.update_todo(Todo { id: todo.id, name: todo.name.clone(), completed: false }) {
                        eprintln!("error marking '{}' todo completed: Error: {}", todo_name, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_name);
                }
            }
            (None, Some(todo_id)) => {
                if let Some(todo) = file_repo.get_todo(todo_id).ok().map(|t| t) {
                    if let Err(err) = file_repo.update_todo(Todo { id: todo.id, name: todo.name.clone(), completed: false }) {
                        eprintln!("error marking '{}' todo completed: Error: {}", todo_id, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_id);
                }
            }
            (Some(_), Some(todo_id)) => {
                if let Some(todo) = file_repo.get_todo(todo_id).ok().map(|t| t) {
                    if let Err(err) = file_repo.update_todo(Todo { id: todo.id, name: todo.name.clone(), completed: false }) {
                        eprintln!("error marking '{}' todo completed: Error: {}", todo_id, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_id);
                }
            }
            (None, None) => panic!("Need either a name or id argument to incomplete a task"),
        },
        Command::List {completed, incomplete, limit} => {
            if let Err(err) = list_all_todos(file_repo, completed, incomplete, limit) {
                eprintln!("error printing all todos: Error: {}", err)
            }
        },
        Command::Togone { name, id } => match ( name, id ) {
            (Some(todo_name), None) => {
                if let Some(todo) = file_repo.get_todo_by_name(todo_name.clone()).ok().map(|t| t) {
                    if let Err(err) = file_repo.delete_todo(Todo { id: todo.id, name: todo.name.clone(), completed: false }) {
                        eprintln!("error deleting '{}': Error: {}", todo_name, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_name);
                }
            }
            (None, Some(todo_id)) => {
                if let Some(todo) = file_repo.get_todo(todo_id).ok().map(|t| t) {
                    if let Err(err) = file_repo.delete_todo(Todo { id: todo.id, name: todo.name.clone(), completed: false }) {
                        eprintln!("error deleting '{}': Error: {}", todo_id, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_id);
                }
            }
            (Some(_), Some(todo_id)) => {
                if let Some(todo) = file_repo.get_todo(todo_id).ok().map(|t| t) {
                    if let Err(err) = file_repo.delete_todo(Todo { id: todo.id, name: todo.name.clone(), completed: false }) {
                        eprintln!("error deleting '{}': Error: {}", todo_id, err);
                    }
                } else {
                    eprintln!("Todo with id '{}' not found.", todo_id);
                }
            }
            (None, None) => panic!("Need either a name or id argument to delete a task"),
        },
        Command::Clean {  } => {
            if let Ok(existing_completed_todos) = file_repo.get_completed_todos(None) {
                if let Err(err) = file_repo.delete_todos(existing_completed_todos) {
                    eprintln!("error cleaning the nudgelyst.yaml file: Error: {}", err)
                }
            } else {
                eprintln!("error cleaning the nudgelyst.yaml file")
            }

            println!("Wow you got a lot done!")
        },
        Command::Wipe {  } => {
            if let Ok(existing_todos) = file_repo.get_todos(None) {
                if let Err(err) = file_repo.delete_todos(existing_todos) {
                    eprintln!("error wiping the nudgelyst.yaml file - consider deleting: Error: {}", err)
                }
            } else {
                eprintln!("error wiping the nudgelyst.yaml file - consider deleting")
            }

            println!("Slate wiped! Time to get some stuff 'todone'!")
        }
    }
}

fn list_last_five(todo_repo: FileTodoRepository) -> Result<(), Box<dyn Error>> {
    let todos = todo_repo.get_todos(Some(5))?;

    list_todos_in_term(todos);

    Ok(())
}

fn list_all_todos(todo_repo: FileTodoRepository, completed_flag: Option<bool>, incomplete_flag: Option<bool>, limit: Option<usize>)
    -> Result<(), Box<dyn Error>> {
    let todos = match (completed_flag, incomplete_flag) {
        (Some(true), None) => {
            todo_repo.get_completed_todos(limit)?
        },
        (Some(false), None) => {
            todo_repo.get_todos(limit)?
        },
        (Some(true), Some(false)) => {
            todo_repo.get_completed_todos(limit)?
        },
        (Some(false), Some(true)) => {
            todo_repo.get_incomplete_todos(limit)?
        },
        (None, Some(true)) => {
            todo_repo.get_incomplete_todos(limit)?
        },
        (None, Some(false)) => {
            todo_repo.get_todos(limit)?
        },
        (None, None) => {
            todo_repo.get_todos(limit)?
        },
        (Some(false), Some(false)) => {
            todo_repo.get_todos(limit)?
        },
        (Some(true), Some(true)) => {
            todo_repo.get_todos(limit)?
        }
    };

    list_todos_in_term(todos);

    Ok(())
}

fn list_todos_in_term(todos: Vec<Todo>) {
    for todo in todos {
        if todo.completed {
            let id_title = format!("{}", "ID: ".green());
            let name_title = format!("{}", ", Name: ".green());
            let colored_id = format!("{}", todo.id.to_string().green());
            let colored_name = format!("{}", todo.name.clone().green());
            let colored_status = "Complete!".green();

            println!("{}{}{}{} - {}", id_title, colored_id, name_title, colored_name, colored_status);
        } else {
            let id_title = format!("{}", "ID: ".yellow());
            let name_title = format!("{}", ", Name: ".yellow());
            let colored_id = format!("{}", todo.id.to_string().yellow());
            let colored_name = format!("{}", todo.name.clone().yellow());
            let colored_status = "Incomplete".yellow();

            println!("{}{}{}{} - {}", id_title, colored_id, name_title, colored_name, colored_status);
        }
    }
}