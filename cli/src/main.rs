use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Parser)]

struct Cli {
    action: String,
    #[arg(num_args(0..))]
    args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    body: String,
}

impl Todo {
    fn create(name: String, body: String, todos: &mut Vec<Todo>) -> Self {
        let new_todo = Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: name.to_string(),
            body: body.to_string(),
        };
        todos.push(new_todo.clone());
        Todo::save_todos(todos).expect("Failed to write file!");
        new_todo
    }
    fn list(todos: &Vec<Todo>) {
        println!("TODOS");
        for todo in todos {
            println!(
                "Todo ID: {}, Title: {}, Body: {}",
                todo.id, todo.title, todo.body
            );
        }
    }
    fn load_json(todos: &mut Vec<Todo>) {
        if !Path::new("todos.json").exists() {
            let mut file = File::create("todos.json").expect("Failed to create file!");
            file.write_all("[]".as_bytes())
                .expect("Write to file failed!");
        }
        let mut file = File::open("todos.json").expect("The file could not be opened!");
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .expect("Veri okunamadı!");

        *todos = serde_json::from_str(&json_data).expect("JSON format is incorrect!");
    }
    fn save_todos(todos: &Vec<Todo>) -> Result<()> {
        let json_data = serde_json::to_string_pretty(todos).unwrap();
        let mut file = File::create("todos.json").expect("Failed to create file!");
        file.write_all(json_data.as_bytes())
            .expect("Write to file failed!");
        Ok(())
    }
}

fn main() {
    let args = Cli::parse();
    let mut todos: Vec<Todo> = vec![];
    Todo::load_json(&mut todos);

    match args.action.as_str() {
        "create" => {
            if args.args.len() >= 2 {
                let title = args.args[0].clone();
                let body = args.args[1..].join(" ");
                Todo::create(title, body, &mut todos);
            } else {
                panic!("At least two arguments are required!")
            }

            Todo::list(&todos);
        }
        "update" => println!("1"),
        "delete" => println!("1"),
        "list" => {
            Todo::list(&todos);
        }
        _ => panic!("Unknown action!"),
    }
}
