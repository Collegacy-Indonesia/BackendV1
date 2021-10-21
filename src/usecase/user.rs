use crate::{
    controllers::user::{AllUserQuery, CreateUserInput, GetUserPath, UpdateUserInput},
    models::user::User,
    repository, Pool,
};
use actix_web::web::{self, Json};

pub fn get_all_user(db: web::Data<Pool>, query: web::Query<AllUserQuery>) -> Vec<User> {
    repository::user::get_all_users(db, query)
}

pub fn get_user_by_id(db: web::Data<Pool>, info: web::Path<GetUserPath>) -> User {
    repository::user::get_user_by_id(db, info.id)
}

pub fn create_user(db: web::Data<Pool>, user: Json<CreateUserInput>) -> User {
    repository::user::create_user(db, user)
}

pub fn update_user(
    db: web::Data<Pool>,
    info: web::Path<GetUserPath>,
    user: Json<UpdateUserInput>,
) -> User {
    repository::user::update_user(db, info.id, user)
}

pub fn delete_user(db: web::Data<Pool>, info: web::Path<GetUserPath>) -> String {
    repository::user::delete_user(db, info.id)
}
