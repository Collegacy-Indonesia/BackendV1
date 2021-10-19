extern crate backend;
extern crate diesel;

use self::backend::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use backend::schema::user::dsl::*;

    let connection = establish_connection();
    let results = user
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for auser in results {
        println!("{:?}", auser.name.unwrap());
        println!("----------\n");
        println!("{:?}", auser.email.unwrap());
    }
}
