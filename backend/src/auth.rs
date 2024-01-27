use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{header, request::Parts, HeaderMap, Request, StatusCode},
    middleware::{from_extractor, Next},
    response::Response,
    Router,
};
use base64ct::{Base64, Encoding};
use entity::users;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;

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
            Some(auth_header) => match token_is_valid(auth_header) {
                Ok(_) => Ok(Self {}),
                Err(e) => Err(StatusCode::UNAUTHORIZED),
            },
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

fn token_is_valid(token: &str) -> Result<(), String> {
    let secret = env::var("SECRET").expect("SECRET not set in env");
    let dec = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims>(token, &dec, &validation) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i32,
    access_level: i16,
    exp: u64,
}

pub fn create_token(user: users::Model) -> Result<String, ()> {
    // let a token last for 24 hours
    let secret = env::var("SECRET").expect("SECRET not set in env");
    let exp = get_sec_since_epoch() + 60 * 60 * 24;
    let claims = Claims {
        user_id: user.id,
        access_level: user.access_level,
        exp,
    };
    let t = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );
    if t.is_ok() {
        Ok(t.unwrap())
    } else {
        Err(())
    }
}

fn get_sec_since_epoch() -> u64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}

pub fn hash_password(pwd: &str) -> String {
    let salt = env::var("SALT").expect("SALT not set");
    let hash = Sha256::digest(salt + pwd);
    Base64::encode_string(&hash)
}
