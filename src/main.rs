use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};
use std::path::Path;

# [derive(Debug)]
struct  TodoItem {
    id: usize,
    description: String,
    completed: bool,
}

impl  TodoItem {
    fn new(id: usize, description: String) -> Self {
        TodoItem {
            id,
            description,
            completed: false,
        }
    }
    
}

fn main() {
    let mut todos: Vec<TodoItem> = load_todos().unwrap_or_else(|_| Vec::new());

    loop {
        println!("\nTodo List:");
        for todo in &todos {
            println!(
                "{}.[{}] - {}",
                todo.id,
                if todo.completed { "x" } else { " " },
                todo.description
            );
        }

        println!("\nOptions:");
        println!("1. Add a new todo");
        println!("2. Complete a todo");
        println!("3. Delete a todo");
        println!("4. Save and exit");

        let choice = get_input("Enter a choice: ");
        match choice.trim() {
            "1" => {
                let description = get_input("Enter the todo description");
                let id = todos.len() + 1;
                todos.push(TodoItem::new(id, description));
            }
            "2" => {
                let id: usize = get_input("Enter the ID of the todo to mark as complete: ")
                    .trim()
                    .parse()
                    .expect("Please enter a valid number");
            if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
                todo.completed = true;
            } else {
                println!("Todo with ID {} not found!", id);
            }
            }
            "3" => {
                let id: usize = get_input("Enter the ID of the todo to delete: ")
                    .trim()
                    .parse()
                    .expect("Please enter a valid number");
                todos.retain(|todo| todo.id != id);
            }
            "4" => {
                save_todos(&todos).expect("Failed to save todos");
                println!("Todos saved! Exiting...");
                break;
            }
            _ => println!("Invaild choice, please try again."),
        }
    }
}

fn get_input(promt: &str) -> String {
    print!("{}", promt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn load_todos() -> Result<Vec<TodoItem>, io::Error> {
    let path = Path::new("todos.txt");
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = read_to_string(path)?;
    let mut todos = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() == 3 {
            let id: usize = parts[0].parse().expect("Invaild ID in file");
            let description = parts[1].to_string();
            let completed: bool = parts[2].parse().expect("Invaild completed status in file");
            todos.push(TodoItem { id, description, completed});
        }
    }

    Ok(todos)
}

fn save_todos(todos: &[TodoItem]) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("todos.txt")?;

    for todo in todos {
        writeln!(file, "{},{},{}", todo.id, todo.description, todo.completed)?;
    }

    Ok(())
}
