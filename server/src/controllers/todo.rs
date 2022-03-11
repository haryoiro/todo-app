use actix_web::{delete, get, patch, post, HttpResponse};

fn test_json() -> serde_json::Value {
    json!({
        "todos": [
            {
                "id": 1,
                "title": "test",
                "completed": false
            },
            {
                "id": 2,
                "title": "test2",
                "completed": false
            },
            {
                "id": 3,
                "title": "test3",
                "completed": false
            }
        ]
    })
}

#[get("/")]
async fn list_todo() -> HttpResponse {
    info!("hello");
    HttpResponse::Ok().json(test_json())
}
#[post("/")]
async fn post_todo() -> String {
    unimplemented!()
}
#[get("/:id")]
async fn get_todo() -> String {
    unimplemented!()
}
#[patch("/:id")]
async fn toggle_todo() -> String {
    unimplemented!()
}
#[delete("/:id")]
async fn delete_todo() -> String {
    unimplemented!()
}
