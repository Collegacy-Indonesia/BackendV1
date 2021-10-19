extern crate diesel;
extern crate dotenv;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod echo;

fn extablish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url must be set in .env");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo::echo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
