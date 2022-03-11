use actix_web::{web, HttpResponse};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::Connection;
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_test_database() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Failed to create pool.")
}

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Error connecting to database")
}

pub fn get_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool.get().map_err(|e| {
        HttpResponse::InternalServerError()
            .body(format!("Could not get database connection: {}", e))
    })
}
