use diesel::PgConnection;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

use crate::schema::todos;
use crate::schema::todos::dsl;

#[derive(Queryable, Deserialize, Serialize, PartialEq, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
#[derive(Serialize, Deserialize)]
pub struct TodoList(pub Vec<Todo>);

impl TodoList {
    pub fn list(conn: &PgConnection) -> Self {
        let result = dsl::todos.load::<Todo>(conn).expect("Error loading todos");
        TodoList(result)
    }
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct NewTodo {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub completed: Option<bool>,
}

impl NewTodo {
    pub fn create(&self, conn: &PgConnection) -> Result<Todo, diesel::result::Error> {
        diesel::insert_into(todos::table)
            .values(self)
            .get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderTodo {
    pub id: Option<bool>,
    pub title: Option<bool>,
    pub completed: Option<bool>,
}
