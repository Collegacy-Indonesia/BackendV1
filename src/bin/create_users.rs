extern crate backend;
extern crate diesel;

use self::backend::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like your name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", name, EOF);
    let mut email = String::new();
    stdin().read_line(&mut email).unwrap();
    let email = &email[..(email.len() - 1)]; // Drop the newline character

    let post = create_user(&connection, name, email);
    println!("\nSaved draft {} with id {}", name, post.id);
}

const EOF: &'static str = "CTRL+D";
