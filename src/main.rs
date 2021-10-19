use actix_web::{App, HttpServer};
mod echo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo::echo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
