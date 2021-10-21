use crate::{
    controllers::user::{CreateUserInput, GetUserPath, UpdateUserInput},
    models::user::User,
    repository::user as user_repository,
    Pool,
};
use actix_web::web::{self, Json};

pub fn get_all_user(db: web::Data<Pool>) -> Vec<User> {
    user_repository::get_users(db)
}

pub fn get_user_by_id(db: web::Data<Pool>, info: web::Path<GetUserPath>) -> User {
    user_repository::get_user_by_id(db, info.id)
}

pub fn create_user(db: web::Data<Pool>, user: Json<CreateUserInput>) -> User {
    user_repository::create_user(db, user)
}

pub fn update_user(
    db: web::Data<Pool>,
    info: web::Path<GetUserPath>,
    user: Json<UpdateUserInput>,
) -> User {
    user_repository::update_user(db, info.id, user)
}

pub fn delete_user(db: web::Data<Pool>, info: web::Path<GetUserPath>) -> String {
    user_repository::delete_user(db, info.id)
}
