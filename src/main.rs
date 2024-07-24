use colored::*;
use ignore::Walk;
use log::warn;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


// TODO: use treesitter for comments parsing
const TODO_PREFIX: &str = "TODO:";

fn trim_todo_prefix(line: &str) -> Option<String> {
    let trimmed_line = line.trim();

    if let Some(pos) = trimmed_line.find(TODO_PREFIX) {
        Some(trimmed_line[(pos + TODO_PREFIX.len())..].to_string())
    } else {
        None
    }
}

fn find_todos(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut todos = Vec::new();

    for line in reader.lines() {
        let line = line?;

        match trim_todo_prefix(&line) {
            Some(new_line) => {
                todos.push(format!(
                    "{}:{}",
                    path.display().to_string().cyan(),
                    new_line
                ));
            }
            None => continue,
        }
    }

    Ok(todos)
}

fn main() -> io::Result<()> {
    env_logger::init_from_env("LOG_LEVEL");

    for entry in Walk::new("./") {
        match entry {
            Ok(entry) => {
                if let Some(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let path = entry.path();
                        match find_todos(path) {
                            Ok(todos) => {
                                for todo in todos {
                                    println!("{}", todo);
                                }
                            }
                            Err(e) => warn!("Error reading {:?}: {}", path, e),
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
