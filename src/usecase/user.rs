use crate::{models::user::User, repository::user as user_repository};

pub fn get_user() -> Vec<User> {
    user_repository::get_users()
}
