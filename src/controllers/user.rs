use crate::{usecase::user as user_usecase, Pool};
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn get_user(db: web::Data<Pool>) -> impl Responder {
    HttpResponse::Ok().json(user_usecase::get_user(db))
}

#[post("/")]
async fn create_user(db: web::Data<Pool>, payload: Json<UserInput>) -> impl Responder {
    HttpResponse::Ok().json(user_usecase::create_user(db, payload))
}

use crate::schema::user;
use crate::utils::serializer::json_time;

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "user"]
pub struct UserInput {
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

fn default_time() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
