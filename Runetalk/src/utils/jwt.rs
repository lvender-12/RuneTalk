use crate::{errors::AppResult, model::config_model::ConfigModel};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: u64,
}

pub fn generate_jwt(uuid: String, email: String, conf: &ConfigModel) -> AppResult<String> {
    let exp = Utc::now() + Duration::seconds(conf.jwt.expiry as i64);
    let claims = Claims {
        sub: uuid,
        email,
        exp: exp.timestamp() as u64,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&conf.jwt.secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str, secret: &str) -> AppResult<Claims> {
    let decode = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(decode.claims)
}

pub fn get_uuid_from_token(token: &str, secret: &str) -> AppResult<String> {
    let claims = decode_jwt(token, secret)?;
    Ok(claims.sub)
}
