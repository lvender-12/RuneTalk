use chrono::Utc;
use uuid::Uuid;

use crate::{
    entity::{Guild, Rift, RiftType},
    model::config_model::{
        ApiConfig, AppConfig, ConfigModel, DbConfig, JwtConfig, RedisConfig, Smtp, Storage,
    },
};

pub fn dummy_config() -> ConfigModel {
    ConfigModel {
        app: AppConfig {
            name: "Test".to_string(),
            host: "localhost".to_string(),
            port: 8080,
        },
        db: DbConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test".to_string(),
            password: "test".to_string(),
            name: "runetalk".to_string(),
        },
        redis: RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            username: String::new(),
            password: String::new(),
        },
        jwt: JwtConfig {
            secret: "test_secret_key_long_enough_for_jwt".to_string(),
            expiry: 3600,
        },
        api: ApiConfig {
            secret: "test-api-secret".to_string(),
        },
        smtp: Smtp {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        },
        storage: Storage {
            path: "./public/user".to_string(),
        },
        allowed_origins: vec!["https://runetalk.app".to_string()],
    }
}

pub fn dummy_guild(owner_id: Uuid, is_public: bool) -> Guild {
    Guild {
        id: Uuid::new_v4(),
        owner_id,
        name: "Test Guild".to_string(),
        description: Some("A test guild".to_string()),
        icon_url: None,
        banner_url: None,
        invite_code: "ABCD1234".to_string(),
        is_public,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

pub fn dummy_rift(guild_id: Uuid) -> Rift {
    Rift {
        id: Uuid::new_v4(),
        guild_id,
        name: "general".to_string(),
        topic: None,
        rift_type: RiftType::Text,
        position: 0,
        is_private: false,
        created_at: Utc::now().naive_utc(),
    }
}
