use actix_web::{App, HttpServer};

mod providers;
mod todo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(todo::routes::todos)
            .service(todo::routes::todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
