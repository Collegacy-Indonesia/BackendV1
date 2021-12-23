use crate::{controllers::user::CreateUserInput, usecase, Pool};
use actix_web::{
    get, post,
    web::{self, Json},
    Error, HttpRequest, HttpResponse,
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
async fn refresh_token(payload: Json<RefreshTokenInput>) -> Result<HttpResponse, Error> {
    usecase::auth::refresh_token(payload)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[post("/profile")]
async fn profile(payload: Json<ProfileInput>) -> Result<HttpResponse, Error> {
    usecase::auth::profile(payload)
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

#[derive(Deserialize, Serialize)]
pub struct ProfileInput {
    pub token: String,
}
