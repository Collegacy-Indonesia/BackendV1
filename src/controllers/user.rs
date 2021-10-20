use crate::usecase::user as user_usecase;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().json(user_usecase::get_user())
}
