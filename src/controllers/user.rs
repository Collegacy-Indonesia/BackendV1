use crate::{usecase, Pool};
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[get("")]
async fn get_all_user(
    db: web::Data<Pool>,
    query: web::Query<AllUserQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::user::get_all_user(db, query)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[get("/{id}")]
async fn get_user_by_id(
    info: web::Path<GetUserPath>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::user::get_user_by_id(db, info)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[post("")]
async fn create_user(
    db: web::Data<Pool>,
    payload: Json<CreateUserInput>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::user::create_user(db, payload)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[put("/{id}")]
async fn update_user(
    info: web::Path<GetUserPath>,
    db: web::Data<Pool>,
    payload: Json<UpdateUserInput>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::user::update_user(db, info, payload)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[delete("/{id}")]
async fn delete_user(
    info: web::Path<GetUserPath>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::user::delete_user(db, info)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

use crate::schema::user;
use crate::utils::serializer::json_time;

#[derive(Deserialize, Debug)]
pub struct GetUserPath {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct AllUserQuery {
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "user"]
pub struct CreateUserInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(default = "default_time", with = "json_time")]
    pub updated_at: NaiveDateTime,
    pub password: String,
}

#[derive(Deserialize, Serialize, Insertable, AsChangeset)]
#[table_name = "user"]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
    pub email_verified: Option<bool>,
    #[serde(default = "default_time", with = "json_time")]
    pub updated_at: NaiveDateTime,
    pub password: Option<String>,
}

fn default_time() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
