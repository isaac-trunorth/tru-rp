use std::env;

use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{header, request::Parts, HeaderMap, Request, StatusCode},
    middleware::{from_extractor, Next},
    response::Response,
    Router,
};
use jsonwebtoken::DecodingKey;

pub struct RequireAuth {}

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) if token_is_valid(auth_header) => Ok(Self {}),
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

fn token_is_valid(token: &str) -> bool {
    let secret = env::var("SECRET").expect("SECRET not set in env");
    let dec = DecodingKey::from_secret(secret.as_bytes());
    token == "test"
}
