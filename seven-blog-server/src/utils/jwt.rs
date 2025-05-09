use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,  // user id
    pub exp: i64,  // expiration time
    pub iat: i64,  // issued at
}

pub fn create_token(user_id: i32, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = (now + Duration::days(7)).timestamp();
    let claims = Claims {
        sub: user_id,
        exp,
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
} 