use crate::controllers::user::UserInput;
use crate::models::user::User;
use crate::Pool;
use actix_web::web::{self, Json};
use chrono::Local;
use diesel::prelude::*;

pub fn create_user<'a>(db: web::Data<Pool>, user: Json<UserInput>) -> User {
    use crate::schema::user;
    let user = user.into_inner();

    let conn = &db.get().unwrap();

    diesel::insert_into(user::table)
        .values(user)
        .execute(conn)
        .expect("Error saving new user");

    user::table.order(user::id.desc()).first(conn).unwrap()
}

pub fn get_users(db: web::Data<Pool>) -> Vec<User> {
    use crate::schema::user::dsl::*;

    let connection = &db.get().unwrap();
    let results = user
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    results
}
