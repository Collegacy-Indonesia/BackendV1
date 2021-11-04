#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate env_logger;

mod middlewares;

use actix_web::{middleware::Logger, web, App, HttpServer};

use ::r2d2::PooledConnection;
use actix_web_httpauth::middleware::HttpAuthentication;
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
pub type UnwrappedPool = PooledConnection<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url must be set in .env");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let result = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(middlewares::validator);
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(pool.clone())
            .service(
                // -- TODO = wrap in auth by simply .wrap(auth)
                web::scope("/users")
                    .service(controllers::user::get_user_by_id)
                    .service(controllers::user::update_user)
                    .service(controllers::user::create_user)
                    .service(controllers::user::delete_user)
                    .service(controllers::user::get_all_user),
            )
            .service(web::scope("/auth").service(controllers::auth::login))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await;

    result
}
