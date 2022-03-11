use diesel::{prelude::*, sql_query};
use server::configs::database::init_test_database;
use server::models::Todo;
use server::schema::todos;

fn main() {
    let connection = init_test_database();
    sql_query("TRUNCATE TABLE todos restart identity")
        .execute(&connection)
        .expect("Error truncating table");
}
