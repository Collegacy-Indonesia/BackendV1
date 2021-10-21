use crate::{usecase::user as user_usecase, Pool};
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AllUserQuery {
    pub limit: Option<i64>,
}

#[get("/")]
async fn get_all_user(db: web::Data<Pool>, query: web::Query<AllUserQuery>) -> impl Responder {
    HttpResponse::Ok().json(user_usecase::get_all_user(db, query))
}

#[get("/{id}")]
async fn get_user_by_id(info: web::Path<GetUserPath>, db: web::Data<Pool>) -> impl Responder {
    HttpResponse::Ok().json(user_usecase::get_user_by_id(db, info))
}

#[post("/")]
async fn create_user(db: web::Data<Pool>, payload: Json<CreateUserInput>) -> impl Responder {
    HttpResponse::Ok().json(user_usecase::create_user(db, payload))
}

#[put("/{id}")]
async fn update_user(
    info: web::Path<GetUserPath>,
    db: web::Data<Pool>,
    payload: Json<UpdateUserInput>,
) -> impl Responder {
    println!("{:?}", info);
    HttpResponse::Ok().json(user_usecase::update_user(db, info, payload))
}

#[delete("/{id}")]
async fn delete_user(info: web::Path<GetUserPath>, db: web::Data<Pool>) -> impl Responder {
    HttpResponse::Ok().body(user_usecase::delete_user(db, info))
}

use crate::schema::user;
use crate::utils::serializer::json_time;

#[derive(Deserialize, Debug)]
pub struct GetUserPath {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "user"]
pub struct CreateUserInput {
    pub name: String,
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
