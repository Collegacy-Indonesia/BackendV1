use crate::{
    controllers::user::UserInput, models::user::User, repository::user as user_repository, Pool,
};
use actix_web::web::{self, Json};

pub fn get_user(db: web::Data<Pool>) -> Vec<User> {
    user_repository::get_users(db)
}

pub fn create_user(db: web::Data<Pool>, user: Json<UserInput>) -> User {
    user_repository::create_user(db, user)
}
