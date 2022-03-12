extern crate server;
use actix_web::{
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpServer,
};
use server::{configs::database::establish_connection, route::todo_scope};
use std::env;

const PORT: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    println!("\nServer ready at http://{}", PORT);

    HttpServer::new(|| {
        App::new()
            // enable logger %r - response first line, %s - response status
            .wrap(Logger::new("%r %s"))
            // enable normalize path - always redirect to trailing slash
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .app_data(web::Data::new(establish_connection()))
            .service(todo_scope())
    })
    .bind(&PORT)?
    .run()
    .await
}
