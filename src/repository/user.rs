extern crate bcrypt;

use crate::controllers::user::{AllUserQuery, CreateUserInput, UpdateUserInput};
use crate::models::user::User;
use crate::UnwrappedPool;
use actix_web::web::{self, Json};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

pub fn create_user<'a>(
    connection: &UnwrappedPool,
    user_data: Json<CreateUserInput>,
) -> Result<User, diesel::result::Error> {
    use crate::schema::user;
    let mut user_data = user_data.into_inner();

    let hashed = hash(user_data.password, DEFAULT_COST).unwrap();
    user_data.password = hashed;

    let insert_respond = diesel::insert_into(user::table)
        .values(user_data)
        .execute(connection);

    match insert_respond {
        Err(e) => return Err(e),
        _ => {}
    }

    let final_user = user::table.order(user::id.desc()).first(connection);

    final_user
}

pub fn update_user<'a>(
    connection: &UnwrappedPool,
    user_id: i32,
    user_data: Json<UpdateUserInput>,
) -> Result<User, diesel::result::Error> {
    use crate::schema::user;
    let mut user_data = user_data.into_inner();

    if user_data.password.is_some() {
        let hashed = hash(user_data.password.unwrap(), DEFAULT_COST).unwrap();
        user_data.password = Some(hashed);
    }

    let update_respond = diesel::update(user::table)
        .filter(user::id.eq(user_id))
        .set(&user_data)
        .execute(connection);

    match update_respond {
        Err(e) => return Err(e),
        _ => {}
    }

    user::table.order(user::id.desc()).first(connection)
}

pub fn get_all_users(
    connection: &UnwrappedPool,
    q: web::Query<AllUserQuery>,
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::user;

    let mut query = user::table.into_boxed();

    if let Some(limit) = q.limit {
        query = query.limit(limit);
    };

    let results = query.load::<User>(connection);

    results
}

pub fn get_user_by_id(
    connection: &UnwrappedPool,
    user_id: i32,
) -> Result<User, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    user.filter(id.eq_all(user_id)).first(connection)
}

pub fn get_user_by_email(
    connection: &UnwrappedPool,
    user_email: String,
) -> Result<User, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    user.filter(email.eq_all(user_email)).first(connection)
}

pub fn delete_user(
    connection: &UnwrappedPool,
    user_id: i32,
) -> Result<String, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    let delete_respond = diesel::delete(user.filter(id.eq_all(user_id))).execute(connection);

    match delete_respond {
        Ok(num_deleted) => return Ok(format!("Successfully deleted {} users", num_deleted)),
        Err(e) => return Err(e),
    };
}
