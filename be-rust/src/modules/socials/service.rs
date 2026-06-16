use crate::{
    entity::{Guild, GuildRole, Rift},
    errors::{AppResult, AuthError, DbError},
    modules::socials::{
        dto::{
            CreateRiftDto, EditGuildDto, EditRiftDto, GuildDto, InviteLinkResponse,
        },
        repository::SocialRepository,
    },
    utils::invite_code::{build_invite_link, generate_invite_code},
};
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait SocialService: Send + Sync {
    async fn create_guild_service(&self, dto: GuildDto, user_id: Uuid) -> AppResult<Guild>;
    async fn get_guild_service(&self, guild_id: Uuid, user_id: Uuid) -> AppResult<Guild>;
    async fn edit_guild_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        dto: EditGuildDto,
    ) -> AppResult<Guild>;
    async fn delete_guild_service(&self, guild_id: Uuid, user_id: Uuid) -> AppResult<()>;
    async fn create_rift_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        dto: CreateRiftDto,
    ) -> AppResult<Rift>;
    async fn edit_rift_service(
        &self,
        guild_id: Uuid,
        rift_id: Uuid,
        user_id: Uuid,
        dto: EditRiftDto,
    ) -> AppResult<Rift>;
    async fn delete_rift_service(
        &self,
        guild_id: Uuid,
        rift_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()>;
    async fn join_guild_service(&self, invite_code: &str, user_id: Uuid) -> AppResult<Guild>;
    async fn get_invite_link_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        origin: &str,
    ) -> AppResult<InviteLinkResponse>;
    async fn regenerate_invite_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        origin: &str,
    ) -> AppResult<InviteLinkResponse>;
}

pub struct SocialServiceImpl {
    pub repo: Arc<dyn SocialRepository>,
}

impl SocialServiceImpl {
    pub fn new(repo: Arc<dyn SocialRepository>) -> Self {
        Self { repo }
    }

    async fn require_role(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        allowed: &[GuildRole],
    ) -> AppResult<GuildRole> {
        let role = self
            .repo
            .get_member_role_repo(guild_id, user_id)
            .await?
            .ok_or(AuthError::Forbidden)?;

        if allowed.contains(&role) {
            Ok(role)
        } else {
            Err(AuthError::Forbidden.into())
        }
    }

    async fn can_access_guild(&self, guild: &Guild, user_id: Uuid) -> AppResult<()> {
        let role = self.repo.get_member_role_repo(guild.id, user_id).await?;

        if role.is_some() || guild.is_public {
            Ok(())
        } else {
            Err(AuthError::Forbidden.into())
        }
    }

    fn invite_response(guild: &Guild, origin: &str) -> InviteLinkResponse {
        InviteLinkResponse {
            invite_code: guild.invite_code.clone(),
            invite_link: build_invite_link(origin, &guild.invite_code),
        }
    }
}

#[async_trait]
impl SocialService for SocialServiceImpl {
    async fn create_guild_service(&self, dto: GuildDto, user_id: Uuid) -> AppResult<Guild> {
        let invite_code = generate_invite_code();
        self.repo.create_guild_repo(dto, user_id, invite_code).await
    }

    async fn get_guild_service(&self, guild_id: Uuid, user_id: Uuid) -> AppResult<Guild> {
        let guild = self.repo.find_guild_by_id_repo(guild_id).await?;
        self.can_access_guild(&guild, user_id).await?;
        Ok(guild)
    }

    async fn edit_guild_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        dto: EditGuildDto,
    ) -> AppResult<Guild> {
        self.require_role(
            guild_id,
            user_id,
            &[GuildRole::Owner, GuildRole::Admin],
        )
        .await?;
        self.repo.edit_guild_repo(guild_id, dto).await
    }

    async fn delete_guild_service(&self, guild_id: Uuid, user_id: Uuid) -> AppResult<()> {
        self.require_role(guild_id, user_id, &[GuildRole::Owner])
            .await?;
        self.repo.delete_guild_repo(guild_id).await
    }

    async fn create_rift_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        dto: CreateRiftDto,
    ) -> AppResult<Rift> {
        self.require_role(
            guild_id,
            user_id,
            &[GuildRole::Owner, GuildRole::Admin],
        )
        .await?;
        self.repo.create_rift_repo(guild_id, dto).await
    }

    async fn edit_rift_service(
        &self,
        guild_id: Uuid,
        rift_id: Uuid,
        user_id: Uuid,
        dto: EditRiftDto,
    ) -> AppResult<Rift> {
        self.require_role(
            guild_id,
            user_id,
            &[GuildRole::Owner, GuildRole::Admin],
        )
        .await?;
        self.repo.edit_rift_repo(guild_id, rift_id, dto).await
    }

    async fn delete_rift_service(
        &self,
        guild_id: Uuid,
        rift_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        self.require_role(
            guild_id,
            user_id,
            &[GuildRole::Owner, GuildRole::Admin],
        )
        .await?;
        self.repo.delete_rift_repo(guild_id, rift_id).await
    }

    async fn join_guild_service(&self, invite_code: &str, user_id: Uuid) -> AppResult<Guild> {
        let guild = self.repo.find_guild_by_invite_code_repo(invite_code).await?;

        if self
            .repo
            .get_member_role_repo(guild.id, user_id)
            .await?
            .is_some()
        {
            return Err(DbError::conflict("Guild member").into());
        }

        self.repo
            .add_guild_member_repo(guild.id, user_id, GuildRole::Member)
            .await?;

        Ok(guild)
    }

    async fn get_invite_link_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        origin: &str,
    ) -> AppResult<InviteLinkResponse> {
        self.require_role(
            guild_id,
            user_id,
            &[GuildRole::Owner, GuildRole::Admin],
        )
        .await?;

        let guild = self.repo.find_guild_by_id_repo(guild_id).await?;
        Ok(Self::invite_response(&guild, origin))
    }

    async fn regenerate_invite_service(
        &self,
        guild_id: Uuid,
        user_id: Uuid,
        origin: &str,
    ) -> AppResult<InviteLinkResponse> {
        self.require_role(guild_id, user_id, &[GuildRole::Owner])
            .await?;

        let invite_code = generate_invite_code();
        let guild = self
            .repo
            .update_invite_code_repo(guild_id, invite_code)
            .await?;

        Ok(Self::invite_response(&guild, origin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entity::RiftType,
        errors::{AppError, DbError},
        modules::socials::{
            dto::{CreateRiftDto, EditGuildDto, EditRiftDto, GuildDto},
            repository::MockSocialRepository,
        },
        testing::fixtures::{dummy_guild, dummy_rift},
    };
    use mockall::predicate::*;
    use std::sync::Arc;

    fn service(mock_repo: MockSocialRepository) -> SocialServiceImpl {
        SocialServiceImpl::new(Arc::new(mock_repo))
    }

    #[tokio::test]
    async fn create_guild_success() {
        let owner_id = Uuid::new_v4();
        let guild = dummy_guild(owner_id, true);
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_create_guild_repo()
            .withf(move |dto, uid, code| {
                dto.name == "Test Guild" && dto.is_public && *uid == owner_id && code.len() == 8
            })
            .times(1)
            .returning(move |_, _, _| Ok(guild.clone()));

        let svc = service(mock_repo);
        let dto = GuildDto {
            name: "Test Guild".to_string(),
            description: Some("A test guild".to_string()),
            is_public: true,
        };

        let result = svc.create_guild_service(dto, owner_id).await.unwrap();
        assert_eq!(result.name, "Test Guild");
        assert_eq!(result.owner_id, owner_id);
    }

    #[tokio::test]
    async fn get_guild_success_for_member() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(user_id, false);
        let guild_id = guild.id;
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_find_guild_by_id_repo()
            .with(eq(guild_id))
            .times(1)
            .returning(move |_| Ok(guild.clone()));

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Member)));

        let svc = service(mock_repo);
        let result = svc.get_guild_service(guild_id, user_id).await.unwrap();
        assert_eq!(result.id, guild_id);
    }

    #[tokio::test]
    async fn get_guild_success_for_public_non_member() {
        let owner_id = Uuid::new_v4();
        let viewer_id = Uuid::new_v4();
        let guild = dummy_guild(owner_id, true);
        let guild_id = guild.id;
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_find_guild_by_id_repo()
            .with(eq(guild_id))
            .times(1)
            .returning(move |_| Ok(guild.clone()));

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(viewer_id))
            .times(1)
            .returning(|_, _| Ok(None));

        let svc = service(mock_repo);
        let result = svc.get_guild_service(guild_id, viewer_id).await.unwrap();
        assert_eq!(result.id, guild_id);
    }

    #[tokio::test]
    async fn get_guild_forbidden_for_private_non_member() {
        let owner_id = Uuid::new_v4();
        let viewer_id = Uuid::new_v4();
        let guild = dummy_guild(owner_id, false);
        let guild_id = guild.id;
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_find_guild_by_id_repo()
            .with(eq(guild_id))
            .times(1)
            .returning(move |_| Ok(guild.clone()));

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(viewer_id))
            .times(1)
            .returning(|_, _| Ok(None));

        let svc = service(mock_repo);
        let err = svc.get_guild_service(guild_id, viewer_id).await.unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Forbidden)));
    }

    #[tokio::test]
    async fn edit_guild_success_for_admin() {
        let admin_id = Uuid::new_v4();
        let guild = dummy_guild(admin_id, true);
        let guild_id = guild.id;
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(admin_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Admin)));

        mock_repo
            .expect_edit_guild_repo()
            .with(eq(guild_id), always())
            .times(1)
            .returning(move |_, _| Ok(guild.clone()));

        let svc = service(mock_repo);
        let dto = EditGuildDto {
            name: "Updated Guild".to_string(),
            description: None,
            icon_url: None,
            banner_url: None,
            is_public: true,
        };

        let result = svc
            .edit_guild_service(guild_id, admin_id, dto)
            .await
            .unwrap();
        assert_eq!(result.id, guild_id);
    }

    #[tokio::test]
    async fn edit_guild_forbidden_for_member() {
        let member_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(member_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Member)));

        let svc = service(mock_repo);
        let dto = EditGuildDto {
            name: "Updated Guild".to_string(),
            description: None,
            icon_url: None,
            banner_url: None,
            is_public: true,
        };

        let err = svc
            .edit_guild_service(guild_id, member_id, dto)
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Forbidden)));
    }

    #[tokio::test]
    async fn delete_guild_success_for_owner() {
        let owner_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(owner_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Owner)));

        mock_repo
            .expect_delete_guild_repo()
            .with(eq(guild_id))
            .times(1)
            .returning(|_| Ok(()));

        let svc = service(mock_repo);
        svc.delete_guild_service(guild_id, owner_id).await.unwrap();
    }

    #[tokio::test]
    async fn delete_guild_forbidden_for_admin() {
        let admin_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(admin_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Admin)));

        let svc = service(mock_repo);
        let err = svc
            .delete_guild_service(guild_id, admin_id)
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Forbidden)));
    }

    #[tokio::test]
    async fn create_rift_success_for_admin() {
        let admin_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(admin_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Admin)));

        mock_repo
            .expect_create_rift_repo()
            .with(eq(guild_id), always())
            .times(1)
            .returning(move |_, _| Ok(rift.clone()));

        let svc = service(mock_repo);
        let dto = CreateRiftDto {
            name: "voice".to_string(),
            topic: None,
            rift_type: RiftType::Voice,
            is_private: false,
        };

        let result = svc
            .create_rift_service(guild_id, admin_id, dto)
            .await
            .unwrap();
        assert_eq!(result.guild_id, guild_id);
    }

    #[tokio::test]
    async fn edit_rift_success_for_owner() {
        let owner_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let rift_id = rift.id;
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(owner_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Owner)));

        mock_repo
            .expect_edit_rift_repo()
            .with(eq(guild_id), eq(rift_id), always())
            .times(1)
            .returning(move |_, _, _| Ok(rift.clone()));

        let svc = service(mock_repo);
        let dto = EditRiftDto {
            name: Some("announcements".to_string()),
            topic: None,
            rift_type: Some(RiftType::Announcement),
            position: None,
            is_private: None,
        };

        let result = svc
            .edit_rift_service(guild_id, rift_id, owner_id, dto)
            .await
            .unwrap();
        assert_eq!(result.id, rift_id);
    }

    #[tokio::test]
    async fn delete_rift_success_for_admin() {
        let admin_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift_id = Uuid::new_v4();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(admin_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Admin)));

        mock_repo
            .expect_delete_rift_repo()
            .with(eq(guild_id), eq(rift_id))
            .times(1)
            .returning(|_, _| Ok(()));

        let svc = service(mock_repo);
        svc.delete_rift_service(guild_id, rift_id, admin_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn join_guild_success() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(Uuid::new_v4(), true);
        let guild_id = guild.id;
        let invite_code = guild.invite_code.clone();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_find_guild_by_invite_code_repo()
            .with(eq(invite_code.clone()))
            .times(1)
            .returning(move |_| Ok(guild.clone()));

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(None));

        mock_repo
            .expect_add_guild_member_repo()
            .with(eq(guild_id), eq(user_id), eq(GuildRole::Member))
            .times(1)
            .returning(|_, _, _| Ok(()));

        let svc = service(mock_repo);
        let result = svc
            .join_guild_service(&invite_code, user_id)
            .await
            .unwrap();
        assert_eq!(result.id, guild_id);
    }

    #[tokio::test]
    async fn join_guild_conflict_when_already_member() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(Uuid::new_v4(), true);
        let guild_id = guild.id;
        let invite_code = guild.invite_code.clone();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_find_guild_by_invite_code_repo()
            .with(eq(invite_code.clone()))
            .times(1)
            .returning(move |_| Ok(guild.clone()));

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Member)));

        let svc = service(mock_repo);
        let err = svc
            .join_guild_service(&invite_code, user_id)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            AppError::Db(DbError::Conflict { entity: "Guild member" })
        ));
    }

    #[tokio::test]
    async fn get_invite_link_success_for_admin() {
        let admin_id = Uuid::new_v4();
        let guild = dummy_guild(admin_id, true);
        let guild_id = guild.id;
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(admin_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Admin)));

        mock_repo
            .expect_find_guild_by_id_repo()
            .with(eq(guild_id))
            .times(1)
            .returning(move |_| Ok(guild.clone()));

        let svc = service(mock_repo);
        let result = svc
            .get_invite_link_service(guild_id, admin_id, "https://runetalk.app")
            .await
            .unwrap();

        assert_eq!(result.invite_code, "ABCD1234");
        assert_eq!(
            result.invite_link,
            "https://runetalk.app/invite/ABCD1234"
        );
    }

    #[tokio::test]
    async fn get_invite_link_forbidden_for_member() {
        let member_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(member_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Member)));

        let svc = service(mock_repo);
        let err = svc
            .get_invite_link_service(guild_id, member_id, "https://runetalk.app")
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Forbidden)));
    }

    #[tokio::test]
    async fn regenerate_invite_success_for_owner() {
        let owner_id = Uuid::new_v4();
        let mut guild = dummy_guild(owner_id, true);
        let guild_id = guild.id;
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(owner_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Owner)));

        mock_repo
            .expect_update_invite_code_repo()
            .with(eq(guild_id), always())
            .times(1)
            .returning(move |_, code| {
                guild.invite_code = code;
                Ok(guild.clone())
            });

        let svc = service(mock_repo);
        let result = svc
            .regenerate_invite_service(guild_id, owner_id, "https://runetalk.app/")
            .await
            .unwrap();

        assert_eq!(result.invite_code.len(), 8);
        assert_eq!(
            result.invite_link,
            format!("https://runetalk.app/invite/{}", result.invite_code)
        );
    }

    #[tokio::test]
    async fn regenerate_invite_forbidden_for_admin() {
        let admin_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock_repo = MockSocialRepository::new();

        mock_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(admin_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Admin)));

        let svc = service(mock_repo);
        let err = svc
            .regenerate_invite_service(guild_id, admin_id, "https://runetalk.app")
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Forbidden)));
    }
}
