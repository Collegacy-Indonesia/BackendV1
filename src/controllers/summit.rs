use crate::{usecase, Pool};
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[get("")]
async fn get_all_summit_registrants(
    db: web::Data<Pool>,
    query: web::Query<AllSummitRegistrantQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::summit::get_all_summit_registrants(db, query)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[get("/{id}")]
async fn get_summit_registrants(
    info: web::Path<GetSummitRegistrantPath>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::summit::get_summit_registrant_by_id(db, info)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[post("")]
async fn create_summit_registrants(
    db: web::Data<Pool>,
    payload: Json<CreateSummitRegistrantInput>,
) -> Result<HttpResponse, actix_web::Error> {
    usecase::summit::create_summit_registrant(db, payload)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

use crate::schema::summit_registrant;

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "summit_registrant"]
pub struct CreateSummitRegistrantInput {
    pub no_hp: String,
    pub universitas: String,
    pub jurusan: String,
    pub batch: i32,
    pub role: String,
    pub link_drive: String,
}

#[derive(Deserialize, Debug)]
pub struct GetSummitRegistrantPath {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct AllSummitRegistrantQuery {
    pub limit: Option<i64>,
}
