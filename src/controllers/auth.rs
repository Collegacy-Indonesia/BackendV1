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

#[post("/refresh-token")]
async fn refresh_token(
    db: web::Data<Pool>,
    payload: Json<RefreshTokenInput>,
) -> Result<HttpResponse, Error> {
    usecase::auth::refresh_token(payload)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[derive(Deserialize, Serialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RefreshTokenInput {
    pub refresh_token: String,
}
