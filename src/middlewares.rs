use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserClaims {
    pub email: String,
    pub id: i32,
    pub exp: usize,
}

fn validate_token(token: String) -> bool {
    let key = b"secret";
    let validation = Validation {
        ..Default::default()
    };

    let token_data = match decode::<UserClaims>(&token, &DecodingKey::from_secret(key), &validation)
    {
        Ok(_) => true,
        Err(_) => false,
    };

    token_data
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    // keep for later usage of pool to middleware:
    // let pool = req.app_data::<web::Data<Pool>>().unwrap();
    if validate_token(credentials.token().to_string()) {
        Ok(req)
    } else {
        Err(AuthenticationError::from(config).into())
    }
}
