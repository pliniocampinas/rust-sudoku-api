use actix_web::{web, App, HttpServer};

mod sudoku_solver;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/solve", web::get().to(|| async { "Hello World!" }))
            .service(sudoku_solver::handle)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}