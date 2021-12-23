use crate::{
    controllers::{
        auth::{LoginInput, ProfileInput, RefreshTokenInput},
        user::CreateUserInput,
    },
    middlewares::UserClaims,
    repository, Pool,
};
use actix_web::{
    error::ErrorBadRequest,
    web::{self, Json},
    Error, HttpRequest,
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> Result<Token, Error> {
    let user =
        match repository::user::get_user_by_email(&db.get().unwrap(), payload.email.to_string()) {
            Err(e) => return Err(ErrorBadRequest(e)),
            Ok(user) => user,
        };

    let user_claims = UserClaims {
        id: user.id,
        email: user.email,
        exp: (Utc::now() + Duration::minutes(30)).timestamp() as usize,
    };

    match verify(payload.password.to_string(), &user.password) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(result) => match result {
            false => return Err(ErrorBadRequest("Password Incorrect!")),
            _ => {}
        },
    };

    let token = generate_access_and_refresh_token(user_claims);

    token
}

pub fn register(db: web::Data<Pool>, payload: Json<CreateUserInput>) -> Result<Token, Error> {
    let user = match repository::user::create_user(&db.get().unwrap(), payload) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(user) => user,
    };

    let user_claims = UserClaims {
        id: user.id,
        email: user.email,
        exp: (Utc::now() + Duration::minutes(30)).timestamp() as usize,
    };

    let token = generate_access_and_refresh_token(user_claims);

    token
}

pub fn refresh_token(payload: Json<RefreshTokenInput>) -> Result<Token, Error> {
    let key = b"secret";
    let refresh_key = b"anotherkey";
    let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };

    let refresh_token_data = match decode::<UserClaims>(
        &payload.refresh_token,
        &DecodingKey::from_secret(refresh_key),
        &validation,
    ) {
        Ok(t) => t,
        Err(e) => return Err(ErrorBadRequest(e)),
    };

    let user_claims = UserClaims {
        id: refresh_token_data.claims.id,
        email: refresh_token_data.claims.email,
        exp: (Utc::now() + Duration::minutes(30)).timestamp() as usize,
    };

    let token = match encode::<UserClaims>(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => t,
        Err(e) => return Err(ErrorBadRequest(e)),
    };

    let auth_token = Token {
        token,
        refresh_token: payload.refresh_token.to_string(),
    };

    Ok(auth_token)
}

// This will also put the refresh token on database
fn generate_access_and_refresh_token(user: UserClaims) -> Result<Token, Error> {
    let key = b"secret";
    let refresh_key = b"anotherkey";
    let token =
        match encode::<UserClaims>(&Header::default(), &user, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(e) => return Err(ErrorBadRequest(e)),
        };

    let refresh_token = match encode::<UserClaims>(
        &Header::default(),
        &user,
        &EncodingKey::from_secret(refresh_key),
    ) {
        Ok(t) => t,
        Err(e) => return Err(ErrorBadRequest(e)),
    };

    let auth_token = Token {
        token,
        refresh_token,
    };

    Ok(auth_token)
}

pub fn profile(payload: Json<ProfileInput>) -> Result<UserClaims, Error> {
    let token = payload.token.to_string();
    print!("{:?}", token);

    let key = b"secret";
    let validation = Validation {
        ..Default::default()
    };

    let token_data = match decode::<UserClaims>(&token, &DecodingKey::from_secret(key), &validation)
    {
        Ok(t) => t,
        Err(e) => return Err(ErrorBadRequest(e)),
    };

    Ok(token_data.claims)
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}
