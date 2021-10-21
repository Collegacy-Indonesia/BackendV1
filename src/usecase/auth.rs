use actix_web::web::{self, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{controllers::auth::LoginInput, models::user::User, repository, Pool};

pub fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> Token {
    let user = repository::user::get_user_by_email(db, payload.email.to_string());
    generate_token(user)
}

fn generate_token(user: User) -> Token {
    // Todo : create password verification, create error
    let key = b"secret";
    let token = match encode(&Header::default(), &user, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(),
    };
    let refresh_token = match encode(&Header::default(), &user, &EncodingKey::from_secret(key)) {
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