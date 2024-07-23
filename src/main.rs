use colored::*;
use ignore::Walk;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn find_todos(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut todos = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("TODO:") {
            todos.push(format!(
                "{}:{}",
                path.display().to_string().cyan(),
                line.trim().trim_start_matches("// TODO:")
            ));
        }
    }

    Ok(todos)
}

fn main() -> io::Result<()> {
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
                            Err(e) => eprintln!("Error reading {:?}: {}", path, e),
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
