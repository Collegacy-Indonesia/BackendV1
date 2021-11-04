use actix_web::web::{self, Json};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::{auth::LoginInput, user::CreateUserInput},
    middlewares::UserClaims,
    repository, Pool, UnwrappedPool,
};

pub fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> Token {
    let user = repository::user::get_user_by_email(&db.get().unwrap(), payload.email.to_string());
    let user_claims = UserClaims {
        email: user.email,
        exp: 1000000000000000,
    };
    let token = generate_token(
        &db.get().unwrap(),
        user_claims,
        payload.password.to_string(),
    );
    token
}

// pub fn register(db: web::Data<Pool>, payload: Json<LoginInput>) -> Token {
//     let user = repository::user::create_user(
//         &db.get().unwrap(),
//         serde_json::to_string(&serializedUser).unwrap(),
//     );
//     let user_claims = UserClaims {
//         email: user.email,
//         exp: 1000000000000000,
//     };
//     let token = generate_token(
//         &db.get().unwrap(),
//         user_claims,
//         payload.password.to_string(),
//     );
//     token
// }

fn generate_token(db: &UnwrappedPool, user: UserClaims, password: String) -> Token {
    // Todo : create password verification, create error
    let user_data = repository::user::get_user_by_email(db, user.email.clone());
    let valid = verify(password, &user_data.password);

    println!("{:?}", valid);

    let validval = valid.map(|_| true).map_err(|_| false);

    println!("{:?}", validval);

    let key = b"secret";
    let token =
        match encode::<UserClaims>(&Header::default(), &user, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(_) => panic!(),
        };
    let refresh_token =
        match encode::<UserClaims>(&Header::default(), &user, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(_) => panic!(),
        };

    let auth_token = Token {
        token,
        refresh_token,
    };

    auth_token
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}
