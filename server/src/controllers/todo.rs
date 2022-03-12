use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::{
    configs::{
        database::{get_pool_handler, PgPool},
        errors::ServerError,
    },
    handlers::todos as todo_handler,
    models::{NewTodo, OrderTodo, Todo},
    schema::todos as todo_schema,
};

#[get("/")]
async fn list_todo(pool: web::Data<PgPool>, query: web::Query<OrderTodo>) -> HttpResponse {
    let todos = todo_handler::list_all(pool, query).await;
    let todos = match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            error!("Error loading todos: {}", e);
            return HttpResponse::InternalServerError().body("Error loading todos");
        }
    };
    todos
}

#[post("/")]
async fn post_todo(
    pool: web::Data<PgPool>,
    new_todo: web::Json<NewTodo>,
) -> Result<HttpResponse, ServerError> {
    let conn = get_pool_handler(pool).unwrap();
    let res = new_todo.create(&conn);
    match res {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(e) => {
            error!("Error creating todo: {}", e);
            Err(ServerError::GenericError("Error creating todo".to_string()))
        }
    }
}

#[get("/{id}/")]
async fn get_todo(pool: web::Data<PgPool>, req: HttpRequest) -> Result<HttpResponse, ServerError> {
    let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
    let conn = get_pool_handler(pool).unwrap();
    todo_schema::dsl::todos
        .find(id)
        .first::<Todo>(&conn)
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(|e| {
            error!("Error loading todo: {}", e);
            ServerError::GenericError("Error loading todo".to_string())
        })
}

/// toggle todo status
/// PATCH /todos/:id/toggle
#[patch("/{id}/")]
async fn toggle_todo(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
    let conn = get_pool_handler(pool).unwrap();
    diesel::update(todo_schema::dsl::todos.find(id))
        .set(todo_schema::dsl::completed.eq(diesel::dsl::not(todo_schema::dsl::completed)))
        .execute(&conn)
        .map(|_| HttpResponse::Ok().json(json!({ "message": "Todo updated" })))
        .map_err(|e| {
            error!("Error updating todo: {}", e);
            ServerError::GenericError("Error updating todo".to_string())
        })
}

#[delete("/{id}/")]
async fn delete_todo(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
    let conn = get_pool_handler(pool).unwrap();
    diesel::delete(todo_schema::dsl::todos.find(id))
        .execute(&conn)
        .map(|_| HttpResponse::NoContent().json(json!({ "message": "Todo deleted" })))
        .map_err(|e| {
            error!("Error deleting todo: {}", e);
            ServerError::GenericError("Error deleting todo".to_string())
        })
}
