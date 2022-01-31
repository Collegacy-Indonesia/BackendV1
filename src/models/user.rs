#![allow(unused)]
#![allow(clippy::all)]

use crate::utils::serializer::{json_time, option_json_time};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
    pub email_verified: bool,
    pub image: Option<String>,
    #[serde(with = "json_time")]
    pub created_at: NaiveDateTime,
    #[serde(with = "json_time")]
    pub updated_at: NaiveDateTime,
}

use crate::schema::user;

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub updated_at: NaiveDateTime,
}
