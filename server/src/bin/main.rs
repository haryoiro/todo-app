extern crate server;
use actix_web::{
    middleware::{Logger, NormalizePath, TrailingSlash},
    App, HttpServer,
};
use server::route::{dev_scope, todo_scope};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%r %s"))
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .service(todo_scope())
            .service(dev_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
