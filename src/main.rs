#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use dotenv::dotenv;
use std::env;
pub mod controllers;
pub mod models;
pub mod repository;
pub mod schema;
pub mod usecase;
pub mod utils;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url must be set in .env");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new().data(pool.clone()).service(
            web::scope("/users")
                .service(controllers::user::get_user_by_id)
                .service(controllers::user::update_user)
                .service(controllers::user::create_user)
                .service(controllers::user::delete_user)
                .service(controllers::user::get_all_user),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
