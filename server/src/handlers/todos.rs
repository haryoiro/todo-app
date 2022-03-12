use actix_web::web;
use diesel::prelude::*;

use crate::{
    configs::database::{get_pool_handler, PgPool},
    models::{OrderTodo, Todo, TodoList},
    schema::todos as todo_schema,
};

pub async fn list_all(
    pool: web::Data<PgPool>,
    query: web::Query<OrderTodo>,
) -> Result<TodoList, diesel::result::Error> {
    let conn = get_pool_handler(pool).expect("Error loading todos");
    let mut query_sql = todo_schema::table.order(todo_schema::id.asc()).into_boxed();

    if query.id.is_some() {
        if query.id.unwrap() {
            query_sql = query_sql.order(todo_schema::id.asc());
        } else {
            query_sql = query_sql.order(todo_schema::id.desc());
        }
    }

    if query.title.is_some() {
        if query.title.unwrap() {
            query_sql = query_sql.order(todo_schema::title.asc());
        } else {
            query_sql = query_sql.order(todo_schema::title.desc());
        }
    }

    let todos = query_sql.load::<Todo>(&conn);

    match todos {
        Ok(todos) => Ok(TodoList(todos)),
        Err(e) => Err(e),
    }
}
