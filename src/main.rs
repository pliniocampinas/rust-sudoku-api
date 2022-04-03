use actix_web::{web, App, HttpServer};

mod greet;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet::handle)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}