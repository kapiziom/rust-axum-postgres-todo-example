use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

const VALID_TOKEN_HOURS: i64 = 24;

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data)
}

pub fn generate_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(VALID_TOKEN_HOURS))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))?;
    Ok(token)
}