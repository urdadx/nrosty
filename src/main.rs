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
    query: String,
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

fn edit_todo(file_path: &str, new_todo: Todo) -> Result<()> {}

fn get_current_time() -> String {
    let now = chrono::Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() -> Result<()> {
    let args = CLI::parse();
    let file_path = "todos.json";
    let command = args.command.as_str();

    // build a new todo
    let timestamp = get_current_time();
    let new_todo = Todo {
        title: args.query,
        completed: false,
        created_at: timestamp.to_owned(),
        deleted_at: "--- ---".to_owned(),
    };

    match command {
        "add" => {
            add_new_todo(file_path, new_todo)?;
        }
        _ => {
            println!("Invalid command");
        }
    }

    Ok(())
}
