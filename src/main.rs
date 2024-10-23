use anyhow::{Context, Result};
use chrono;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    completed: bool,
    created_at: String,
    deleted_at: String,
}

#[derive(Parser)]
struct CLI {
    command: String,
    query: Option<String>,
    todo_id: Option<usize>,
}

fn read_json_file(file_path: &str) -> Result<Vec<Todo>> {
    let file = File::open(file_path).context("Failed to open the JSON file")?;
    let reader = BufReader::new(file);

    let todos: Vec<Todo> = serde_json::from_reader(reader).context("Failed to deserialize JSON")?;

    Ok(todos)
}

fn write_json_file(file_path: &str, todos: &Vec<Todo>) -> Result<()> {
    let file = File::create(file_path).context("Failed to create the JSON file")?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, todos).context("Failed to write JSON to file")?;

    Ok(())
}

fn add_new_todo(file_path: &str, new_todo: Todo) -> Result<()> {
    // Read the existing todos from the file
    let mut todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());

    todos.push(new_todo);
    write_json_file(file_path, &todos)?;

    Ok(())
}

fn edit_todo(file_path: &str, todo_index: usize, query: &str) -> Result<()> {
    let mut todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());

    if let Some(todo) = todos.get_mut(todo_index) {
        todo.title = query.to_owned();
    } else {
        println!("Todo with the specified index does not exist");
        return Ok(());
    }

    write_json_file(file_path, &todos)?;
    println!("Todo updated successfully");

    Ok(())
}

fn mark_as_completed(file_path: &str, todo_index: usize) -> Result<()> {
    let mut todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());

    if let Some(todo) = todos.get_mut(todo_index) {
        todo.completed = true;
    } else {
        println!("Todo with the specified index does not exist");
        return Ok(());
    }

    write_json_file(file_path, &todos)?;
    println!("Todo was marked as completed");

    Ok(())
}

fn delete_todo(file_path: &str, todo_index: usize) -> Result<()> {
    let mut todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());

    if todo_index < todos.len() {
        todos.remove(todo_index);
        write_json_file(file_path, &todos)?;
        println!("Todo deleted successfully");
    } else {
        println!("Todo with the specified index does not exist");
    }

    Ok(())
}

fn get_current_time() -> String {
    let now = chrono::Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() -> Result<()> {
    let args = CLI::parse();
    let file_path = "todos.json";
    let command = args.command.as_str();

    match command {
        "add" => {
            if let Some(query) = args.query {
                // build a new todo
                let timestamp = get_current_time();
                let new_todo = Todo {
                    title: query,
                    completed: false,
                    created_at: timestamp.to_owned(),
                    deleted_at: "--- ---".to_owned(),
                };
                add_new_todo(file_path, new_todo)?;
            } else {
                println!("Query is required for adding a new todo");
            }
        }

        "edit" => {
            if let Some(todo_id) = args.todo_id {
                if let Some(query) = args.query {
                    edit_todo(file_path, todo_id, &query)?;
                } else {
                    println!("Query is required for editing a todo");
                }
            } else {
                println!("Todo ID is required for editing");
            }
        }
        "delete" => {
            if let Some(todo_id) = args.todo_id {
                delete_todo(file_path, todo_id)?;
            } else {
                println!("Todo ID is required for editing");
            }
        }

        "done" => {
            if let Some(todo_id) = args.todo_id {
                mark_as_completed(file_path, todo_id)?;
            } else {
                println!("Todo ID is required for marking as completed");
            }
        }
        _ => {
            println!("Invalid command");
        }
    }

    Ok(())
}
