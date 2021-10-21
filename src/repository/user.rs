use crate::controllers::user::{CreateUserInput, UpdateUserInput};
use crate::models::user::User;
use crate::Pool;
use actix_web::web::{self, Json};
use diesel::prelude::*;

pub fn create_user<'a>(db: web::Data<Pool>, user_data: Json<CreateUserInput>) -> User {
    use crate::schema::user;
    let user_data = user_data.into_inner();

    let conn = &db.get().unwrap();

    diesel::insert_into(user::table)
        .values(user_data)
        .execute(conn)
        .expect("Error saving new user");

    user::table.order(user::id.desc()).first(conn).unwrap()
}

pub fn update_user<'a>(
    db: web::Data<Pool>,
    user_id: i32,
    user_data: Json<UpdateUserInput>,
) -> User {
    use crate::schema::user;
    let user_data = user_data.into_inner();

    let conn = &db.get().unwrap();

    diesel::update(user::table)
        .filter(user::id.eq(user_id))
        .set(&user_data)
        .execute(conn)
        .expect("Error updating user");

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

pub fn get_user_by_id(db: web::Data<Pool>, user_id: i32) -> User {
    use crate::schema::user::dsl::*;

    let connection = &db.get().unwrap();
    let result = user
        .filter(id.eq_all(user_id))
        .first(connection)
        .expect("Error loading users");

    result
}

pub fn delete_user(db: web::Data<Pool>, user_id: i32) -> String {
    use crate::schema::user::dsl::*;

    let connection = &db.get().unwrap();
    let num_deleted = diesel::delete(user.filter(id.eq_all(user_id)))
        .execute(connection)
        .expect("Error loading users");

    format!("Successfully deleted {} users", num_deleted)
}
