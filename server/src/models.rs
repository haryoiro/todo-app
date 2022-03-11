use serde_derive::{Deserialize, Serialize};

use crate::schema::todos;

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
}
#[derive(Queryable, Deserialize, Serialize, PartialEq, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
