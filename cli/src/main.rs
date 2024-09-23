use clap::Parser;
use prettytable::color;
use prettytable::Attr;
use prettytable::Cell;
use prettytable::Row;
use prettytable::Table;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Parser)]
struct Cli {
    action: String,
    #[arg(num_args(0..))]
    args: Vec<String>,
}

struct CliCommand {
    name: String,
    command: String,
    information: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    body: String,
}

impl Todo {
    fn create(title: String, body: String, todos: &mut Vec<Todo>) -> Self {
        let new_todo = Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            body: body.to_string(),
        };
        todos.push(new_todo.clone());
        Todo::save_todos(todos).expect("Failed to write file!");
        new_todo
    }
    fn update(todos: &mut Vec<Todo>, id: String, todo: Todo) {
        if let Some(index) = todos.iter().position(|t| t.id == id) {
            todos[index] = todo;
        }
        Todo::save_todos(todos).expect("Failed to write file!");
    }
    fn delete(todos: &mut Vec<Todo>, id: String) {
        todos.retain(|t| t.id != id);
        Todo::save_todos(todos).expect("Failed to write file!");
    }
    fn list(todos: &mut Vec<Todo>) {
        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new("ID").with_style(Attr::ForegroundColor(color::MAGENTA)),
            Cell::new("TITLE").with_style(Attr::ForegroundColor(color::MAGENTA)),
            Cell::new("BODY").with_style(Attr::ForegroundColor(color::MAGENTA)),
        ]));

        for todo in todos {
            table.add_row(Row::new(vec![
                Cell::new(&todo.id.to_string()).with_style(Attr::ForegroundColor(color::CYAN)),
                Cell::new(&todo.title.to_string()).with_style(Attr::ForegroundColor(color::CYAN)),
                Cell::new(&todo.body.to_string()).with_style(Attr::ForegroundColor(color::CYAN)),
            ]));
        }

        table.printstd();
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

    clear_terminal();

    match args.action.as_str() {
        "create" => {
            if args.args.len() >= 2 {
                let title = args.args[0].clone();
                let body = args.args[1].clone();
                Todo::create(title, body, &mut todos);
            } else {
                panic!("At least two arguments are required!")
            }

            Todo::list(&mut todos);
        }
        "update" => {
            if args.args.len() >= 3 {
                let id = args.args[0].clone();
                let title = args.args[1].clone();
                let body = args.args[2].clone();
                Todo::update(
                    &mut todos,
                    id.clone(),
                    Todo {
                        id: id.clone(),
                        title,
                        body,
                    },
                );
            } else {
                panic!("At least three arguments are required!")
            }

            Todo::list(&mut todos);
        }
        "delete" => {
            if args.args.len() >= 1 {
                let id = args.args[0].clone();
                Todo::delete(&mut todos, id.clone());
            } else {
                panic!("At least an argument is required!")
            }

            Todo::list(&mut todos);
        }
        "list" => {
            Todo::list(&mut todos);
        }
        "help" => help(),
        _ => panic!("Unknown action!"),
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn help() {
    let commands: Vec<CliCommand> = vec![
        CliCommand {
            name: String::from("create"),
            command: String::from("cli.exe create \"<title>\" \"<body>\""),
            information: String::from("Used to create a new TODO. <title> is the title, <body> is the description."),
        },
        CliCommand {
            name: String::from("update"),
            command: String::from("cli.exe update \"<id>\" \"<title>\" \"<body>\""),
            information: String::from("Used to update an existing TODO. <id> is the ID of the TODO, <title> is the new title, <body> is the new description."),
        },
        CliCommand {
            name: String::from("delete"),
            command: String::from("cli.exe delete \"<id>\""),
            information: String::from("Used to delete a TODO with the specified ID. <id> is the ID of the TODO to be deleted."),
        },
        CliCommand {
            name: String::from("list"),
            command: String::from("cli.exe list"),
            information: String::from("Lists all existing TODOs."),
        },
        CliCommand {
            name: String::from("help"),
            command: String::from("cli.exe help"),
            information: String::from("Displays the list of available commands and their descriptions."),
        },
    ];

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Name").with_style(Attr::ForegroundColor(color::MAGENTA)),
        Cell::new("Command").with_style(Attr::ForegroundColor(color::MAGENTA)),
        Cell::new("Information").with_style(Attr::ForegroundColor(color::MAGENTA)),
    ]));

    for command in commands {
        table.add_row(Row::new(vec![
            Cell::new(&command.name.to_string()).with_style(Attr::ForegroundColor(color::CYAN)),
            Cell::new(&command.command.to_string()).with_style(Attr::ForegroundColor(color::CYAN)),
            Cell::new(&command.information.to_string())
                .with_style(Attr::ForegroundColor(color::CYAN)),
        ]));
    }

    table.printstd();
}
