use crate::{controllers::user::CreateUserInput, usecase, Pool};
use actix_web::{
    post,
    web::{self, Json},
    Error, HttpResponse,
};
// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[post("/login")]
async fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> Result<HttpResponse, Error> {
    usecase::auth::login(db, payload)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[post("/register")]
async fn register(
    db: web::Data<Pool>,
    payload: Json<CreateUserInput>,
) -> Result<HttpResponse, Error> {
    usecase::auth::register(db, payload)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[derive(Deserialize, Serialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
