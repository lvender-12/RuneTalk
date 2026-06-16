use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigModel {
    pub app: AppConfig,
    pub db: DbConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub api: ApiConfig,
    pub smtp: Smtp,
    pub allowed_origins: Vec<String>,
    pub storage: Storage,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Smtp {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Storage {
    pub path: String,
}
