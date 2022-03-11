use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_test_database() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Failed to create pool.")
}

pub fn connection_manager() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
