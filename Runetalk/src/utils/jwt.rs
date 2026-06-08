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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config_model::{
        ApiConfig, AppConfig, ConfigModel, DbConfig, JwtConfig, RedisConfig, Smtp, Storage,
    };

    fn dummy_config(expiry_seconds: u64) -> ConfigModel {
        ConfigModel {
            app: AppConfig {
                name: "TestApp".to_string(),
                host: "localhost".to_string(),
                port: 8080,
            },
            db: DbConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "postgres".to_string(),
                password: "password".to_string(),
                name: "testdb".to_string(),
                ssl_mode: "disable".to_string(),
            },
            redis: RedisConfig {
                host: "localhost".to_string(),
                port: 6379,
                username: "".to_string(),
                password: "".to_string(),
            },
            jwt: JwtConfig {
                secret: "super-secret-key-that-is-long-enough-for-hs256".to_string(),
                expiry: expiry_seconds,
            },
            api: ApiConfig {
                secret: "api-secret".to_string(),
            },
            smtp: Smtp {
                email: "test@example.com".to_string(),
                password: "password".to_string(),
            },
            storage: Storage {
                path: "./public/user".to_string(),
            },
            allowed_origins: vec!["http://localhost:3000".to_string()],
        }
    }

    #[test]
    fn test_jwt_generation_and_decoding() {
        let conf = dummy_config(3600);
        let uuid = "550e8400-e29b-41d4-a716-446655440000".to_string();
        let email = "user@example.com".to_string();

        let token = generate_jwt(uuid.clone(), email.clone(), &conf).unwrap();
        let claims = decode_jwt(&token, &conf.jwt.secret).unwrap();

        assert_eq!(claims.sub, uuid);
        assert_eq!(claims.email, email);

        let uuid_extracted = get_uuid_from_token(&token, &conf.jwt.secret).unwrap();
        assert_eq!(uuid_extracted, uuid);
    }

    #[test]
    fn test_jwt_invalid_secret() {
        let conf = dummy_config(3600);
        let uuid = "550e8400-e29b-41d4-a716-446655440000".to_string();
        let email = "user@example.com".to_string();

        let token = generate_jwt(uuid, email, &conf).unwrap();
        let decode_err = decode_jwt(&token, "wrong-secret-key-that-is-incorrect-12345");
        assert!(decode_err.is_err());
    }
}
