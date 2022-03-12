pub mod controllers {
    pub mod dev;
    pub mod todo;
}
pub mod configs {
    pub mod database;
    pub mod errors;
}
pub mod handlers {
    pub mod todos;
}
pub mod models;
pub mod route;
pub mod schema;

#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
