use crate::{
    controllers::user::{AllUserQuery, CreateUserInput, GetUserPath, UpdateUserInput},
    models::user::User,
    repository, Pool,
};
use actix_web::{
    error::ErrorBadRequest,
    web::{self, Json},
};

pub fn get_all_user(
    db: web::Data<Pool>,
    query: web::Query<AllUserQuery>,
) -> Result<Vec<User>, actix_web::Error> {
    repository::user::get_all_users(&db.get().unwrap(), query)
        .map(|all_user| all_user)
        .map_err(|err| ErrorBadRequest(err))
}

pub fn get_user_by_id(
    db: web::Data<Pool>,
    info: web::Path<GetUserPath>,
) -> Result<User, actix_web::Error> {
    repository::user::get_user_by_id(&db.get().unwrap(), info.id)
        .map(|user| user)
        .map_err(|err| ErrorBadRequest(err))
}

pub fn create_user(
    db: web::Data<Pool>,
    user: Json<CreateUserInput>,
) -> Result<User, actix_web::Error> {
    repository::user::create_user(&db.get().unwrap(), user)
        .map(|user| user)
        .map_err(|err| ErrorBadRequest(err))
}

pub fn update_user(
    db: web::Data<Pool>,
    info: web::Path<GetUserPath>,
    user: Json<UpdateUserInput>,
) -> Result<User, actix_web::Error> {
    repository::user::update_user(&db.get().unwrap(), info.id, user)
        .map(|response| response)
        .map_err(|err| ErrorBadRequest(err))
}

pub fn delete_user(
    db: web::Data<Pool>,
    info: web::Path<GetUserPath>,
) -> Result<String, actix_web::Error> {
    repository::user::delete_user(&db.get().unwrap(), info.id)
        .map(|response| response)
        .map_err(|err| ErrorBadRequest(err))
}
