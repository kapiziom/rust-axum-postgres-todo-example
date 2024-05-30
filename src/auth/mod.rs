pub mod jwt;

use axum::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::{request::Parts, StatusCode};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

use crate::auth::jwt::{Claims, validate_token};

pub struct AuthenticatedUser {
    pub user_id: i32,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
    where
        S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.get("Authorization").ok_or(StatusCode::UNAUTHORIZED)?;
        let token = headers.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

        let token_data = validate_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let parsed: Result<i32, _> = token_data.claims.sub.parse();

        Ok(AuthenticatedUser {
            user_id: parsed.unwrap_or_default(),
        })
    }
}