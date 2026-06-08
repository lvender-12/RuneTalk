use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Debug)]
pub struct EditUserDto {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub bio: Option<String>,
}

#[derive(Deserialize, Debug, FromRow, Serialize)]
pub struct EditUserResponseDto {
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub bio: Option<String>,
    pub is_verified: bool,
}
