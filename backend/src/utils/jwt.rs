use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const _EXPIRATION_HOURS: i64 = 24;

pub fn _create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(_EXPIRATION_HOURS))
        .expect("Invalid time")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn _verify_jwt(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims.sub)
}
