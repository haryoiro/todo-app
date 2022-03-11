use crate::configs::database::init_test_database;
use crate::models::{NewTodo, Todo};
use crate::schema::todos;
use diesel::prelude::*;

pub fn list_all_todos() -> Vec<Todo> {
    let connection = init_test_database();

    let todos = todos::dsl::todos
        .load::<Todo>(&connection)
        .expect("Error loading todos");

    todos
}

pub fn insert_todo(title: String) -> Todo {
    let connection = init_test_database();

    let new_todo = NewTodo { title: title };
    let todo = diesel::insert_into(todos::dsl::todos)
        .values(&new_todo)
        .get_result::<Todo>(&connection)
        .expect("Error saving new todo");

    todo
}
