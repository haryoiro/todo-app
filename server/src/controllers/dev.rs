use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/")]
async fn get_index() -> &'static str { "Hello world!" }

#[get("/hello")]
async fn get_hello() -> &'static str { "Hello world!" }
#[get("/json")]
async fn get_json() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "Hello world!"
    })))
}
