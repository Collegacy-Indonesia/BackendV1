#![allow(unused)]
#![allow(clippy::all)]

use crate::utils::serializer::{json_time, option_json_time};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Debug, PartialEq)]
pub struct SummitRegistrant {
    pub id: i32,
    pub user_id: i32,
    pub no_hp: String,
    pub universitas: String,
    pub jurusan: String,
    pub batch: i32,
    pub role: String,
    pub link_drive: String,
}
