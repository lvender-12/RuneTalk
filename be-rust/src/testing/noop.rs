use async_trait::async_trait;
use axum::extract::Multipart;
use uuid::Uuid;

use crate::{
    app::AppState,
    common::response::ApiResponse,
    errors::AppResult,
    modules::{
        auth::{
            dto::{LoginDto, RegisterDto, VerifyOtpDto},
            service::AuthService,
        },
        user::{
            dto::{EditUserResponseDto, FriendRequest, ProfileUser},
            service::UserService,
        },
        ws::{
            dto::{SendEchoDto, SendWhisperDto},
            service::WsService,
        },
    },
};

#[derive(Clone)]
pub struct NoopAuthService;

#[async_trait]
impl AuthService for NoopAuthService {
    async fn register_service(&self, _: RegisterDto) -> AppResult<ApiResponse> {
        unreachable!("auth service should not be called in social tests")
    }

    async fn send_verification_otp(&self, _: &str) -> AppResult<ApiResponse> {
        unreachable!("auth service should not be called in social tests")
    }

    async fn verification_otp(&self, _: VerifyOtpDto) -> AppResult<ApiResponse> {
        unreachable!("auth service should not be called in social tests")
    }

    async fn login_service(&self, _: LoginDto) -> AppResult<String> {
        unreachable!("auth service should not be called in social tests")
    }
}

#[derive(Clone)]
pub struct NoopUserService;

#[async_trait]
impl UserService for NoopUserService {
    async fn edit_user_service(
        &self,
        _: &AppState,
        _: Multipart,
        _: Uuid,
    ) -> AppResult<EditUserResponseDto> {
        unreachable!("user service should not be called in social tests")
    }

    async fn profile_user(&self, _: Uuid) -> AppResult<ProfileUser> {
        unreachable!("user service should not be called in social tests")
    }

    async fn add_friend_service(&self, _: &str, _: Uuid) -> AppResult<(FriendRequest, Uuid)> {
        unreachable!("user service should not be called in social tests")
    }

    async fn list_incoming_requests_service(
        &self,
        _: Uuid,
    ) -> AppResult<Vec<FriendRequest>> {
        unreachable!("user service should not be called in social tests")
    }

    async fn accept_friend_service(&self, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("user service should not be called in social tests")
    }

    async fn reject_friend_service(&self, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("user service should not be called in social tests")
    }

    async fn block_friend_service(&self, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("user service should not be called in social tests")
    }

    async fn is_ally_service(&self, _: Uuid, _: Uuid) -> AppResult<bool> {
        unreachable!("user service should not be called in social tests")
    }

    async fn remove_ally_service(&self, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("user service should not be called in social tests")
    }
}

#[derive(Clone)]
pub struct NoopSocialService;

#[async_trait]
impl crate::modules::socials::service::SocialService for NoopSocialService {
    async fn create_guild_service(
        &self,
        _: crate::modules::socials::dto::GuildDto,
        _: Uuid,
    ) -> AppResult<crate::entity::Guild> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn get_guild_service(
        &self,
        _: Uuid,
        _: Uuid,
    ) -> AppResult<crate::entity::Guild> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn edit_guild_service(
        &self,
        _: Uuid,
        _: Uuid,
        _: crate::modules::socials::dto::EditGuildDto,
    ) -> AppResult<crate::entity::Guild> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn delete_guild_service(&self, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn create_rift_service(
        &self,
        _: Uuid,
        _: Uuid,
        _: crate::modules::socials::dto::CreateRiftDto,
    ) -> AppResult<crate::entity::Rift> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn edit_rift_service(
        &self,
        _: Uuid,
        _: Uuid,
        _: Uuid,
        _: crate::modules::socials::dto::EditRiftDto,
    ) -> AppResult<crate::entity::Rift> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn delete_rift_service(&self, _: Uuid, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn join_guild_service(
        &self,
        _: &str,
        _: Uuid,
    ) -> AppResult<crate::entity::Guild> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn get_invite_link_service(
        &self,
        _: Uuid,
        _: Uuid,
        _: &str,
    ) -> AppResult<crate::modules::socials::dto::InviteLinkResponse> {
        unreachable!("social service should not be called in repository tests")
    }

    async fn regenerate_invite_service(
        &self,
        _: Uuid,
        _: Uuid,
        _: &str,
    ) -> AppResult<crate::modules::socials::dto::InviteLinkResponse> {
        unreachable!("social service should not be called in repository tests")
    }
}

#[derive(Clone)]
pub struct NoopWsService;

#[async_trait]
impl WsService for NoopWsService {
    async fn verify_rift_access(&self, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("ws service should not be called in social tests")
    }

    async fn verify_scroll_access(&self, _: Uuid, _: Uuid) -> AppResult<()> {
        unreachable!("ws service should not be called in social tests")
    }

    async fn send_echo_service(&self, _: SendEchoDto, _: Uuid) -> AppResult<crate::entity::Echo> {
        unreachable!("ws service should not be called in social tests")
    }

    async fn send_whisper_service(
        &self,
        _: SendWhisperDto,
        _: Uuid,
    ) -> AppResult<crate::entity::Whisper> {
        unreachable!("ws service should not be called in social tests")
    }

    async fn scroll_recipient_id(&self, _: Uuid, _: Uuid) -> AppResult<Uuid> {
        unreachable!("ws service should not be called in social tests")
    }

    async fn set_presence(
        &self,
        _: Uuid,
        _: crate::entity::PresenceStatus,
    ) -> AppResult<crate::entity::Presence> {
        unreachable!("ws service should not be called in social tests")
    }
}
