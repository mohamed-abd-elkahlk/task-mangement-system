use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (usually the user's unique ID)
    pub exp: usize,   // Expiration time (in seconds since the epoch)
    pub iat: usize,   // Issued at time (in seconds since the epoch)
    pub role: String, // User role (e.g., "admin", "user")
}

pub fn generate_jwt(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expiration = now
        .checked_add_signed(Duration::seconds(3600 * 24 * 7)) // Token valid for 7 days
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: now.timestamp() as usize,
        role: role.to_string(),
    };

    let secret = env::var("SECRET").expect("Error: please provide SECRET in the .env file");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET").expect("Error: please provide SECRET in the .env file");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}
