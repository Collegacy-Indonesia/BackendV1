extern crate bcrypt;

use crate::controllers::user::{AllUserQuery, CreateUserInput, UpdateUserInput};
use crate::models::user::User;
use crate::UnwrappedPool;
use actix_web::web::{self, Json};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

pub fn create_user<'a>(connection: &UnwrappedPool, user_data: Json<CreateUserInput>) -> User {
    use crate::schema::user;
    let mut user_data = user_data.into_inner();

    let hashed = hash(user_data.password, DEFAULT_COST).unwrap();
    user_data.password = hashed;

    diesel::insert_into(user::table)
        .values(user_data)
        .execute(connection)
        .expect("Error saving new user");

    user::table
        .order(user::id.desc())
        .first(connection)
        .unwrap()
}

pub fn update_user<'a>(
    connection: &UnwrappedPool,
    user_id: i32,
    user_data: Json<UpdateUserInput>,
) -> User {
    use crate::schema::user;
    let mut user_data = user_data.into_inner();

    if user_data.password.is_some() {
        let hashed = hash(user_data.password.unwrap(), DEFAULT_COST).unwrap();
        user_data.password = Some(hashed);
    }

    diesel::update(user::table)
        .filter(user::id.eq(user_id))
        .set(&user_data)
        .execute(connection)
        .expect("Error updating user");

    user::table
        .order(user::id.desc())
        .first(connection)
        .unwrap()
}

pub fn get_all_users(connection: &UnwrappedPool, q: web::Query<AllUserQuery>) -> Vec<User> {
    use crate::schema::user;

    let mut query = user::table.into_boxed();

    if let Some(limit) = q.limit {
        query = query.limit(limit);
    };

    let results = query.load::<User>(connection).expect("error loading user");

    results
}

pub fn get_user_by_id(connection: &UnwrappedPool, user_id: i32) -> User {
    use crate::schema::user::dsl::*;

    let result = user
        .filter(id.eq_all(user_id))
        .first(connection)
        .expect("Error loading users");

    result
}

pub fn get_user_by_email(connection: &UnwrappedPool, user_email: String) -> User {
    use crate::schema::user::dsl::*;

    let result = user
        .filter(email.eq_all(user_email))
        .first(connection)
        .expect("Error loading users");

    result
}

pub fn delete_user(connection: &UnwrappedPool, user_id: i32) -> String {
    use crate::schema::user::dsl::*;

    let num_deleted = diesel::delete(user.filter(id.eq_all(user_id)))
        .execute(connection)
        .expect("Error loading users");

    format!("Successfully deleted {} users", num_deleted)
}
