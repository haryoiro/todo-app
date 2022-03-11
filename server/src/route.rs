use crate::controllers::{
    dev::{get_hello, get_index, get_json},
    todo::{delete_todo, get_todo, list_todo, post_todo, toggle_todo},
};
use actix_web::{web, Scope};

// host:port/api へアクセスしたときのルーティングを設定します。
// App.new().service(api_scope())
pub fn dev_scope() -> Scope {
    web::scope("/dev")
        .service(get_index)
        .service(get_hello)
        .service(get_json)
}
pub fn todo_scope() -> Scope {
    web::scope("/todos")
        .service(list_todo)
        .service(get_todo)
        .service(post_todo)
        .service(toggle_todo)
        .service(delete_todo)
}
