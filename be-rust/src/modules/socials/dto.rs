use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entity::RiftType;

#[derive(Deserialize, Debug, Validate)]
pub struct GuildDto {
    #[validate(length(min = 4, message = "Nama guild minimal harus 4 karakter"))]
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditGuildDto {
    #[validate(length(min = 4, message = "Nama guild minimal harus 4 karakter"))]
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_public: bool,
}

#[derive(Deserialize, Debug, Validate)]
pub struct CreateRiftDto {
    #[validate(length(min = 3, message = "Nama rift minimal harus 3 karakter"))]
    pub name: String,
    pub topic: Option<String>,
    #[serde(default = "default_rift_type")]
    pub rift_type: RiftType,
    pub is_private: bool,
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditRiftDto {
    #[validate(length(min = 3, message = "Nama rift minimal harus 3 karakter"))]
    pub name: Option<String>,
    pub topic: Option<String>,
    pub rift_type: Option<RiftType>,
    pub position: Option<i32>,
    pub is_private: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct InviteLinkResponse {
    pub invite_code: String,
    pub invite_link: String,
}

fn default_rift_type() -> RiftType {
    RiftType::Text
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn guild_dto_validates_name_min_length() {
        let valid = GuildDto {
            name: "Test".to_string(),
            description: None,
            is_public: true,
        };
        assert!(valid.validate().is_ok());

        let invalid = GuildDto {
            name: "abc".to_string(),
            description: None,
            is_public: false,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn edit_guild_dto_validates_name_min_length() {
        let valid = EditGuildDto {
            name: "Test".to_string(),
            description: None,
            icon_url: None,
            banner_url: None,
            is_public: true,
        };
        assert!(valid.validate().is_ok());

        let invalid = EditGuildDto {
            name: "ab".to_string(),
            description: None,
            icon_url: None,
            banner_url: None,
            is_public: false,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn create_rift_dto_validates_name_min_length() {
        let valid = CreateRiftDto {
            name: "general".to_string(),
            topic: None,
            rift_type: RiftType::Text,
            is_private: false,
        };
        assert!(valid.validate().is_ok());

        let invalid = CreateRiftDto {
            name: "ab".to_string(),
            topic: None,
            rift_type: RiftType::Voice,
            is_private: false,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn edit_rift_dto_validates_name_min_length_when_present() {
        let valid = EditRiftDto {
            name: Some("voice".to_string()),
            topic: None,
            rift_type: None,
            position: None,
            is_private: None,
        };
        assert!(valid.validate().is_ok());

        let invalid = EditRiftDto {
            name: Some("ab".to_string()),
            topic: None,
            rift_type: None,
            position: None,
            is_private: None,
        };
        assert!(invalid.validate().is_err());

        let empty = EditRiftDto {
            name: None,
            topic: Some("topic".to_string()),
            rift_type: None,
            position: None,
            is_private: None,
        };
        assert!(empty.validate().is_ok());
    }
}
