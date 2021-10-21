use crate::{usecase, Pool};
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[post("/login")]
async fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> impl Responder {
    HttpResponse::Ok().json(usecase::auth::login(db, payload))
}

// #[post("/register")]
// async fn register(db: web::Data<Pool>, payload: Json<CreateUserInput>) -> impl Responder {
//     HttpResponse::Ok().json(user_usecase::create_user(db, payload))
// }

#[derive(Deserialize, Serialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
