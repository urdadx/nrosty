use anyhow::{Context, Result};
use chrono;
use clap::{Parser, Subcommand};
use comfy_table::{Cell, Color, ContentArrangement, Table};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    completed: bool,
    created_at: String,
    modified_at: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    Edit { todo_id: usize, title: String },
    Delete { todo_id: usize },
    Done { todo_id: usize },
    List,
    Clear,
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
    let mut todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());
    todos.push(new_todo);
    write_json_file(file_path, &todos)?;
    println!("Todo added successfully");
    Ok(())
}

fn edit_todo(file_path: &str, todo_index: usize, title: &str) -> Result<()> {
    let mut todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());

    if let Some(todo) = todos.get_mut(todo_index) {
        todo.title = title.to_owned();
        write_json_file(file_path, &todos)?;
        println!("Todo updated successfully");
    } else {
        println!("Todo with the specified index does not exist");
    }

    Ok(())
}

fn mark_as_completed(file_path: &str, todo_index: usize) -> Result<()> {
    let mut todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());

    if let Some(todo) = todos.get_mut(todo_index) {
        todo.completed = true;
        write_json_file(file_path, &todos)?;
        println!("Todo marked as completed");
    } else {
        println!("Todo with the specified index does not exist");
    }

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

fn list_todos(file_path: &str) -> Result<()> {
    let todos = read_json_file(file_path).unwrap_or_else(|_| Vec::new());

    if todos.is_empty() {
        println!("No todos found...Use the add command to add a new todo");
        return Ok(());
    }

    let mut table = Table::new();

    // Configure the table style
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(120)
        .set_header(vec![
            Cell::new("ID").fg(Color::Green),
            Cell::new("Status").fg(Color::Green),
            Cell::new("Title").fg(Color::Green),
            Cell::new("Date created").fg(Color::Green),
            Cell::new("Date modified").fg(Color::Green),
        ]);

    // Add data rows
    for (index, todo) in todos.iter().enumerate() {
        let status = if todo.completed {
            Cell::new("✓").fg(Color::Green)
        } else {
            Cell::new("☐").fg(Color::Yellow)
        };

        let title = if todo.completed {
            Cell::new(&todo.title).fg(Color::White)
        } else {
            Cell::new(&todo.title).fg(Color::White)
        };

        table.add_row(vec![
            Cell::new(index).fg(Color::Blue),
            status,
            title,
            Cell::new(&todo.created_at).fg(Color::Cyan),
            Cell::new(&todo.modified_at).fg(Color::White),
        ]);
    }

    // Print the table
    println!("{table}");
    let completed_count = todos.iter().filter(|todo| todo.completed).count();
    let uncompleted_count = todos.len() - completed_count;
    println!(
        "\n📊 Total todos: {} | Completed: {} | Uncompleted: {}\n",
        todos.len(),
        completed_count,
        uncompleted_count
    );

    Ok(())
}

fn clear_todos(file_path: &str) -> Result<()> {
    let todos: Vec<Todo> = Vec::new();
    write_json_file(file_path, &todos)?;
    println!("All todos have been cleared");
    Ok(())
}

fn get_current_time() -> String {
    let now = chrono::Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    let file_path = "todos.json";

    match &cli.command {
        Commands::Add { title } => {
            let timestamp = get_current_time();
            let new_todo = Todo {
                title: title.clone(),
                completed: false,
                created_at: timestamp,
                modified_at: "--- --- ---".to_owned(),
            };
            add_new_todo(file_path, new_todo)?;
        }
        Commands::Edit { todo_id, title } => {
            edit_todo(file_path, *todo_id, title)?;
        }
        Commands::Delete { todo_id } => {
            delete_todo(file_path, *todo_id)?;
        }
        Commands::Done { todo_id } => {
            mark_as_completed(file_path, *todo_id)?;
        }
        Commands::List => {
            list_todos(file_path)?;
        }
        Commands::Clear => {
            clear_todos(file_path)?;
        }
    }

    Ok(())
}
