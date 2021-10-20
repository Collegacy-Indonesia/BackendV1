use crate::models::user::{NewUser, User};
use crate::utils::connection::establish_connection;
use chrono::Local;
use diesel::prelude::*;

pub fn create_user<'a>(conn: &MysqlConnection, name: &'a str, email: &'a str) -> User {
    use crate::schema::user;

    let new_user = NewUser {
        name: name.to_string(),
        email: email.to_string(),
        updated_at: Local::now().naive_utc(),
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");

    user::table.order(user::id.desc()).first(conn).unwrap()
}

pub fn get_users() -> Vec<User> {
    use crate::schema::user::dsl::*;

    let connection = establish_connection();
    let results = user
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    results
}
