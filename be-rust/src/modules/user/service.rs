use async_trait::async_trait;
use axum::extract::Multipart;
use std::sync::Arc;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::debug;
use uuid::Uuid;

use crate::{
    app::AppState,
    errors::{AppError, AppResult, DbError, ValidationError},
    modules::user::{
        dto::{EditUserDto, EditUserResponseDto, FriendRequest, ProfileUser},
        repository::UserRepository,
    },
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn edit_user_service(
        &self,
        state: &AppState,
        mut multipart: Multipart,
        id: Uuid,
    ) -> AppResult<EditUserResponseDto>;
    async fn profile_user(&self, id: Uuid) -> AppResult<ProfileUser>;
    async fn add_friend_service(&self, username: &str, id: Uuid) -> AppResult<(FriendRequest, Uuid)>;
    async fn list_incoming_requests_service(&self, user_id: Uuid) -> AppResult<Vec<FriendRequest>>;
    async fn accept_friend_service(&self, from: Uuid, to: Uuid) -> AppResult<()>;
    async fn reject_friend_service(&self, from: Uuid, to: Uuid) -> AppResult<()>;
    async fn block_friend_service(&self, from: Uuid, to: Uuid) -> AppResult<()>;
    async fn is_ally_service(&self, user1: Uuid, user2: Uuid) -> AppResult<bool>;
    async fn remove_ally_service(&self, user1: Uuid, user2: Uuid) -> AppResult<()>;
}

pub struct UserServiceImpl {
    pub repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn edit_user_service(
        &self,
        state: &AppState,
        mut multipart: Multipart,
        id: Uuid,
    ) -> AppResult<EditUserResponseDto> {
        let cwd = std::env::current_dir().unwrap();
        debug!("Aplikasi berjalan di: {:?}", cwd);
        debug!(
            "Path lengkap yang dituju: {:?}",
            format!("{}/{}", state.config.storage.path, id)
        );

        let mut user = self.repo.find_by_id(id).await?;
        let mut edit_user_dto = EditUserDto {
            display_name: None,
            avatar_url: None,
            banner_url: None,
            bio: None,
        };

        while let Some(field) = multipart.next_field().await? {
            let name = field
                .name()
                .ok_or_else(|| ValidationError::Invalid("Field name missing".to_string()))?
                .to_string();

            if let Some(file_name) = field.file_name() {
                let mime = field
                    .content_type()
                    .ok_or_else(|| ValidationError::Invalid("Content-Type missing".to_string()))?;

                if !mime.starts_with("image/") {
                    return Err(ValidationError::Invalid(
                        "File harus berupa gambar atau gif".to_string(),
                    )
                    .into());
                }

                let file_name = file_name.to_string();
                let bytes = field.bytes().await?;
                let dir_path = format!("{}/{}", state.config.storage.path, id);
                tokio::fs::create_dir_all(&dir_path).await?;
                let extension = std::path::Path::new(&file_name)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                let new_file_name = if name == "avatar" {
                    if !extension.is_empty() {
                        format!("avatar.{}", extension)
                    } else {
                        "avatar".to_string()
                    }
                } else if name == "banner" {
                    if !extension.is_empty() {
                        format!("banner.{}", extension)
                    } else {
                        "banner".to_string()
                    }
                } else {
                    file_name.clone()
                };

                let path = format!("{}/{}", dir_path, new_file_name);
                debug!("Mencoba menulis file ke: {}", path);

                if name == "avatar" {
                    if let Some(ref old_avatar) = user.avatar_url {
                        if tokio::fs::metadata(old_avatar).await.is_ok() {
                            debug!("Menghapus avatar lama: {}", old_avatar);
                            let _ = tokio::fs::remove_file(old_avatar).await;
                        }
                    }
                } else if name == "banner" {
                    if let Some(ref old_banner) = user.banner_url {
                        if tokio::fs::metadata(old_banner).await.is_ok() {
                            debug!("Menghapus banner lama: {}", old_banner);
                            let _ = tokio::fs::remove_file(old_banner).await;
                        }
                    }
                }

                let mut file = File::create(&path).await?;
                file.write_all(&bytes).await?;
                debug!("Berhasil menulis file");

                if name == "avatar" {
                    edit_user_dto.avatar_url = Some(path);
                } else if name == "banner" {
                    edit_user_dto.banner_url = Some(path);
                }
            } else {
                let text = field.text().await?;
                debug!("field {} : {}", name, text);
                if name == "display_name" {
                    edit_user_dto.display_name = Some(text);
                } else if name == "bio" {
                    edit_user_dto.bio = Some(text);
                }
            }
        }

        if let Some(display_name) = edit_user_dto.display_name {
            user.display_name = Some(display_name);
        }
        if let Some(avatar) = edit_user_dto.avatar_url {
            user.avatar_url = Some(avatar);
        }
        if let Some(banner) = edit_user_dto.banner_url {
            user.banner_url = Some(banner);
        }
        if let Some(bio) = edit_user_dto.bio {
            user.bio = Some(bio);
        }
        let updated_user = self.repo.edit_user_repo(user).await?;
        Ok(updated_user)
    }

    async fn profile_user(&self, id: Uuid) -> AppResult<ProfileUser> {
        let profile = self.repo.find_by_id(id).await?;
        let profile = ProfileUser {
            id: profile.id,
            username: profile.username,
            email: profile.email,
            avatar_url: profile.avatar_url,
            banner_url: profile.banner_url,
            bio: profile.bio,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        };
        debug!("profile : {:?}", profile);
        Ok(profile)
    }

    async fn add_friend_service(&self, username: &str, id: Uuid) -> AppResult<(FriendRequest, Uuid)> {
        let user = self.repo.find_by_username(username).await?;
        debug!("{:?}", user);

        if let Some(user) = user {
            let request = self.repo.add_friend(id, user.id).await?;
            Ok((request, user.id))
        } else {
            Err(AppError::Db(DbError::not_found("User")))
        }
    }

    async fn list_incoming_requests_service(&self, user_id: Uuid) -> AppResult<Vec<FriendRequest>> {
        let requests = self.repo.list_incoming_requests(user_id).await?;
        Ok(requests)
    }

    async fn accept_friend_service(&self, from: Uuid, to: Uuid) -> AppResult<()> {
        self.repo.accept_friend(from, to).await?;
        Ok(())
    }

    async fn reject_friend_service(&self, from: Uuid, to: Uuid) -> AppResult<()> {
        self.repo.delete_pledge(from, to).await?;
        Ok(())
    }

    async fn block_friend_service(&self, from: Uuid, to: Uuid) -> AppResult<()> {
        self.repo.block_user(from, to).await?;
        Ok(())
    }

    async fn is_ally_service(&self, user1: Uuid, user2: Uuid) -> AppResult<bool> {
        self.repo.is_ally(user1, user2).await
    }

    async fn remove_ally_service(&self, user1: Uuid, user2: Uuid) -> AppResult<()> {
        self.repo.remove_ally(user1, user2).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Adventurer;
    use chrono::Utc;

    fn _dummy_adventurer(username: &str, email: &str) -> Adventurer {
        Adventurer {
            id: Uuid::new_v4(),
            username: username.to_string(),
            display_name: None,
            email: email.to_string(),
            password: "hashed_password".to_string(),
            avatar_url: None,
            banner_url: None,
            bio: None,
            is_verified: true,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
