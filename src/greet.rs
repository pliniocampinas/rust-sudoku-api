use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
pub async fn handle(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}