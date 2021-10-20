#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};
pub mod controllers;
pub mod models;
pub mod repository;
pub mod schema;
pub mod usecase;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    HttpServer::new(|| App::new().service(controllers::user::get_user))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
