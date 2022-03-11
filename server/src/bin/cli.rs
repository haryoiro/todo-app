use diesel::prelude::*;
use server::configs::database::init_test_database;
use server::models::{NewTodo, Todo};
use server::schema::todos as todos_schema;
use std::io::prelude::*;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
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

#[tokio::main]
async fn main() -> io::Result<()> {
    loop {
        let mut action = String::new();
        println!("Select the action you wish to perform from the following");
        println!("1 => List all todos");
        println!("2 => Insert a new todo");
        println!("3 => Delete all todos");
        println!("_ => Quit\n");

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let action = Actions::from_numstring(&action.as_str().trim());
        println!("You selected: {:?}\n", action);
        match action {
            Actions::ListAll => list_all_todos().await,
            Actions::Insert => insert_todo().await,
            Actions::Delete => delete_todos().await,
            Actions::Quit => break,
        }

        write!(io::stdout(), "Press any key to continue....").expect("Failed to write to stdout");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_exact(&mut [0u8; 1])
            .expect("Failed to read stdin");
    }
    Ok(())
}

async fn insert_todo() {
    let connection = init_test_database();
    let mut title = String::new();
    println!("Enter the title of the todo you wish to insert");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let new_todo = NewTodo {
        id: None,
        title: Some(title.trim().to_string()),
        completed: None,
    };
    let todo = diesel::insert_into(todos_schema::table)
        .values(&new_todo)
        .load::<Todo>(&connection)
        .expect("Error saving new todo");
    println!("Saved Todo : \n{:?}\n", todo);
}

async fn list_all_todos() {
    let connection = init_test_database();
    let todos = todos_schema::table
        .load::<Todo>(&connection)
        .expect("Error loading todos");
    if todos.is_empty() {
        println!("No todos found");
    } else {
        for todo in todos {
            println!("{:?}", todo);
        }
    }
}

async fn delete_todos() {
    let connection = init_test_database();
    diesel::sql_query("TRUNCATE TABLE todos restart identity")
        .execute(&connection)
        .expect("Error truncating todos");
}
