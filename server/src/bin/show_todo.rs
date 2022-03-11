use diesel::prelude::*;
use server::configs::database::init_test_database;
use server::models::Todo;
use server::schema::todos;

fn main() {
    let connection = init_test_database();

    let todos = todos::dsl::todos
        .load::<Todo>(&connection)
        .expect("Error loading todos");

    for todo in todos {
        println!("{:?}", todo);
    }
}
