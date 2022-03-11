use diesel::prelude::*;
use server::configs::database::init_test_database;
use server::models::{NewTodo, Todo};
use server::schema::todos as todos_schema;
use std::io;
use std::str::FromStr;

enum Actions {
    ListAll,
    Insert,
    Delete,
    Quit,
}

impl FromStr for Actions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(Actions::ListAll),
            "insert" => Ok(Actions::Insert),
            "delete" => Ok(Actions::Delete),
            "quit" => Ok(Actions::Quit),
            _ => Err(format!("Unknown action: {}", s)),
        }
    }
}

impl Actions {
    fn from_numstring(s: &str) -> Self {
        match s {
            "1" => Actions::ListAll,
            "2" => Actions::Insert,
            "3" => Actions::Delete,
            "4" => Actions::Quit,
            _ => Actions::Quit,
        }
    }
}
fn main() {
    let mut action = String::new();
    loop {
        println!("Select the action you wish to perform from the following");
        println!("1. List all todos");
        println!("2. Insert a new todo");
        println!("3. Delete all todos");

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let action = Actions::from_numstring(&action);
        match action {
            Actions::ListAll => list_all_todos(),
            Actions::Insert => insert_todo(),
            Actions::Delete => delete_todos(),
            Actions::Quit => break,
        }
    }
}

fn insert_todo() {
    let connection = init_test_database();
    let mut title = String::new();
    println!("Enter the title of the todo you wish to insert");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let new_todo = NewTodo {
        title: title.trim().to_string(),
    };
    let todo = diesel::insert_into(todos_schema::table)
        .values(&new_todo)
        .load::<Todo>(&connection)
        .expect("Error saving new todo");
    println!("Saved Todo : {:?}", todo);
}

fn list_all_todos() {
    let connection = init_test_database();
    let todos = todos_schema::table
        .load::<Todo>(&connection)
        .expect("Error loading todos");
    for todo in todos {
        println!("{:?}", todo);
    }
}

fn delete_todos() {
    let connection = init_test_database();
    diesel::delete(todos_schema::table)
        .execute(&connection)
        .expect("Error deleting todos");
}
