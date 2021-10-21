use crate::controllers::user::{AllUserQuery, CreateUserInput, UpdateUserInput};
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

pub fn get_all_users(db: web::Data<Pool>, q: web::Query<AllUserQuery>) -> Vec<User> {
    use crate::schema::user;

    let mut query = user::table.into_boxed();

    if let Some(limit) = q.limit {
        query = query.limit(limit);
    };

    let connection = &db.get().unwrap();
    let results = query.load::<User>(connection).expect("error loading user");

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
