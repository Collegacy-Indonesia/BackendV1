use crate::{
    controllers::{auth::LoginInput, user::CreateUserInput},
    middlewares::UserClaims,
    repository, Pool, UnwrappedPool,
};
use actix_web::{
    error::ErrorBadRequest,
    web::{self, Json},
    Error,
};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> Result<Token, Error> {
    let user = repository::user::get_user_by_email(&db.get().unwrap(), payload.email.to_string());

    let user = match user {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(user) => user,
    };

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

pub fn register(db: web::Data<Pool>, user: Json<CreateUserInput>) -> Result<Token, Error> {
    let user = match repository::user::create_user(&db.get().unwrap(), user) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(user) => user,
    };

    let user_claims = UserClaims {
        email: user.email,
        exp: 1000000000000000,
    };

    let token = generate_token(&db.get().unwrap(), user_claims, user.password.to_string());
    token
}

fn generate_token(db: &UnwrappedPool, user: UserClaims, password: String) -> Result<Token, Error> {
    // Todo : create password verification, create error

    let user_data = match repository::user::get_user_by_email(db, user.email.clone()) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(user) => user,
    };

    match verify(password, &user_data.password) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(result) => match result {
            false => return Err(ErrorBadRequest("Password Incorrect!")),
            _ => {}
        },
    };

    let key = b"secret";
    let token =
        match encode::<UserClaims>(&Header::default(), &user, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(e) => return Err(ErrorBadRequest(e)),
        };

    let refresh_token =
        match encode::<UserClaims>(&Header::default(), &user, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(e) => return Err(ErrorBadRequest(e)),
        };

    let auth_token = Token {
        token,
        refresh_token,
    };

    Ok(auth_token)
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}
