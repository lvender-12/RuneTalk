use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use http::StatusCode;
use validator::Validate;
use uuid::Uuid;

use crate::{
    app::AppState,
    common::{get_uuid::current_user_id, response::ApiResponse},
    errors::AppResult,
    modules::socials::dto::{
        CreateRiftDto, EditGuildDto, EditRiftDto, GuildDto,
    },
};

fn invite_origin(state: &AppState) -> &str {
    state
        .config
        .allowed_origins
        .first()
        .map(String::as_str)
        .unwrap_or("http://localhost:5173")
}

pub async fn create_guild_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<GuildDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;
    let user_id = current_user_id(&jar, &state).await?;
    let guild = state
        .social_service
        .create_guild_service(payload, user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Membuat Guild", guild)),
    ))
}

pub async fn get_guild_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(guild_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    let guild = state
        .social_service
        .get_guild_service(guild_id, user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Mengambil Guild", guild)),
    ))
}

pub async fn edit_guild_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(guild_id): Path<Uuid>,
    Json(payload): Json<EditGuildDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;
    let user_id = current_user_id(&jar, &state).await?;
    let guild = state
        .social_service
        .edit_guild_service(guild_id, user_id, payload)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Mengubah Guild", guild)),
    ))
}

pub async fn delete_guild_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(guild_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    state
        .social_service
        .delete_guild_service(guild_id, user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success_msg("Berhasil Menghapus Guild")),
    ))
}

pub async fn create_rift_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(guild_id): Path<Uuid>,
    Json(payload): Json<CreateRiftDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;
    let user_id = current_user_id(&jar, &state).await?;
    let rift = state
        .social_service
        .create_rift_service(guild_id, user_id, payload)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Membuat Rift", rift)),
    ))
}

pub async fn edit_rift_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path((guild_id, rift_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<EditRiftDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;
    let user_id = current_user_id(&jar, &state).await?;
    let rift = state
        .social_service
        .edit_rift_service(guild_id, rift_id, user_id, payload)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Mengubah Rift", rift)),
    ))
}

pub async fn delete_rift_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path((guild_id, rift_id)): Path<(Uuid, Uuid)>,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    state
        .social_service
        .delete_rift_service(guild_id, rift_id, user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success_msg("Berhasil Menghapus Rift")),
    ))
}

pub async fn join_guild_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(invite_code): Path<String>,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    let guild = state
        .social_service
        .join_guild_service(&invite_code, user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Bergabung ke Guild", guild)),
    ))
}

pub async fn get_invite_link_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(guild_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    let invite = state
        .social_service
        .get_invite_link_service(guild_id, user_id, invite_origin(&state))
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Mengambil Invite Link", invite)),
    ))
}

pub async fn regenerate_invite_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(guild_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    let invite = state
        .social_service
        .regenerate_invite_service(guild_id, user_id, invite_origin(&state))
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Berhasil Memperbarui Invite Link", invite)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_extra::extract::cookie::Cookie;
    use axum_test::TestServer;
    use mockall::predicate::*;
    use std::sync::Arc;

    use crate::{
        common::response::ApiResponse,
        entity::RiftType,
        errors::{AppError, AuthError},
        modules::socials::{
            dto::InviteLinkResponse,
            service::MockSocialService,
        },
        testing::{
            auth_cookie_jar, auth_token, fixtures::{dummy_guild, dummy_rift},
            router::social_test_router, test_app_state,
        },
    };

    async fn parse_ok(result: AppResult<impl axum::response::IntoResponse>) -> (StatusCode, ApiResponse) {
        let response = result.expect("handler should succeed").into_response();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body");
        let body: ApiResponse = serde_json::from_slice(&bytes).expect("json body");
        (status, body)
    }

    #[tokio::test]
    async fn create_guild_handler_success_direct() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(user_id, true);
        let mut mock = MockSocialService::new();

        mock.expect_create_guild_service()
            .times(1)
            .returning(move |_, _| Ok(guild.clone()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);
        let payload = GuildDto {
            name: "Test Guild".to_string(),
            description: None,
            is_public: true,
        };

        let (status, body) =
            parse_ok(create_guild_handler(State(state), jar, Json(payload)).await).await;

        assert_eq!(status, StatusCode::OK);
        assert!(body.success);
        assert_eq!(body.message, "Berhasil Membuat Guild");
    }

    #[tokio::test]
    async fn create_guild_handler_validation_error() {
        let user_id = Uuid::new_v4();
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);
        let payload = GuildDto {
            name: "abc".to_string(),
            description: None,
            is_public: true,
        };

        match create_guild_handler(State(state), jar, Json(payload)).await {
            Err(err) => assert!(matches!(err, AppError::Validation(_))),
            Ok(_) => panic!("expected validation error"),
        }
    }

    #[tokio::test]
    async fn create_guild_handler_unauthorized_without_cookie() {
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let payload = GuildDto {
            name: "Test Guild".to_string(),
            description: None,
            is_public: true,
        };

        match create_guild_handler(State(state), CookieJar::new(), Json(payload)).await {
            Err(err) => assert!(matches!(err, AppError::Auth(AuthError::Unauthorized))),
            Ok(_) => panic!("expected unauthorized error"),
        }
    }

    #[tokio::test]
    async fn create_guild_handler_http_success() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(user_id, true);
        let mut mock = MockSocialService::new();

        mock.expect_create_guild_service()
            .times(1)
            .returning(move |_, _| Ok(guild.clone()));

        let state = test_app_state(Arc::new(mock)).await;
        let token = auth_token(user_id, &state.config);
        let server = TestServer::new(social_test_router(state));

        let response = server
            .post("/guild")
            .add_cookie(Cookie::new("token", token))
            .json(&serde_json::json!({
                "name": "Test Guild",
                "description": null,
                "is_public": true
            }))
            .await;

        response.assert_status_ok();
        let json = response.json::<ApiResponse>();
        assert!(json.success);
        assert_eq!(json.message, "Berhasil Membuat Guild");
    }

    #[tokio::test]
    async fn get_guild_handler_success() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(user_id, true);
        let guild_id = guild.id;
        let mut mock = MockSocialService::new();

        mock.expect_get_guild_service()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(move |_, _| Ok(guild.clone()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let (status, body) =
            parse_ok(get_guild_handler(State(state), jar, Path(guild_id)).await).await;

        assert_eq!(status, StatusCode::OK);
        assert!(body.success);
    }

    #[tokio::test]
    async fn edit_guild_handler_success() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(user_id, true);
        let guild_id = guild.id;
        let mut mock = MockSocialService::new();

        mock.expect_edit_guild_service()
            .with(eq(guild_id), eq(user_id), always())
            .times(1)
            .returning(move |_, _, _| Ok(guild.clone()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);
        let payload = EditGuildDto {
            name: "Updated Guild".to_string(),
            description: None,
            icon_url: None,
            banner_url: None,
            is_public: false,
        };

        let (status, body) = parse_ok(edit_guild_handler(
            State(state),
            jar,
            Path(guild_id),
            Json(payload),
        )
        .await)
        .await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Mengubah Guild");
    }

    #[tokio::test]
    async fn delete_guild_handler_success() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock = MockSocialService::new();

        mock.expect_delete_guild_service()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let (status, body) =
            parse_ok(delete_guild_handler(State(state), jar, Path(guild_id)).await).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Menghapus Guild");
    }

    #[tokio::test]
    async fn create_rift_handler_success() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let mut mock = MockSocialService::new();

        mock.expect_create_rift_service()
            .with(eq(guild_id), eq(user_id), always())
            .times(1)
            .returning(move |_, _, _| Ok(rift.clone()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);
        let payload = CreateRiftDto {
            name: "voice".to_string(),
            topic: None,
            rift_type: RiftType::Voice,
            is_private: false,
        };

        let (status, body) = parse_ok(create_rift_handler(
            State(state),
            jar,
            Path(guild_id),
            Json(payload),
        )
        .await)
        .await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Membuat Rift");
    }

    #[tokio::test]
    async fn edit_rift_handler_success() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let rift_id = rift.id;
        let mut mock = MockSocialService::new();

        mock.expect_edit_rift_service()
            .with(eq(guild_id), eq(rift_id), eq(user_id), always())
            .times(1)
            .returning(move |_, _, _, _| Ok(rift.clone()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);
        let payload = EditRiftDto {
            name: Some("announcements".to_string()),
            topic: None,
            rift_type: Some(RiftType::Announcement),
            position: None,
            is_private: None,
        };

        let (status, body) = parse_ok(edit_rift_handler(
            State(state),
            jar,
            Path((guild_id, rift_id)),
            Json(payload),
        )
        .await)
        .await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Mengubah Rift");
    }

    #[tokio::test]
    async fn delete_rift_handler_success() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift_id = Uuid::new_v4();
        let mut mock = MockSocialService::new();

        mock.expect_delete_rift_service()
            .with(eq(guild_id), eq(rift_id), eq(user_id))
            .times(1)
            .returning(|_, _, _| Ok(()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let (status, body) = parse_ok(delete_rift_handler(
            State(state),
            jar,
            Path((guild_id, rift_id)),
        )
        .await)
        .await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Menghapus Rift");
    }

    #[tokio::test]
    async fn join_guild_handler_success() {
        let user_id = Uuid::new_v4();
        let guild = dummy_guild(Uuid::new_v4(), true);
        let invite_code = guild.invite_code.clone();
        let mut mock = MockSocialService::new();

        mock.expect_join_guild_service()
            .with(eq(invite_code.clone()), eq(user_id))
            .times(1)
            .returning(move |_, _| Ok(guild.clone()));

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let (status, body) = parse_ok(join_guild_handler(
            State(state),
            jar,
            Path(invite_code),
        )
        .await)
        .await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Bergabung ke Guild");
    }

    #[tokio::test]
    async fn get_invite_link_handler_success() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock = MockSocialService::new();

        mock.expect_get_invite_link_service()
            .with(eq(guild_id), eq(user_id), eq("https://runetalk.app"))
            .times(1)
            .returning(|_, _, _| {
                Ok(InviteLinkResponse {
                    invite_code: "ABCD1234".to_string(),
                    invite_link: "https://runetalk.app/invite/ABCD1234".to_string(),
                })
            });

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let (status, body) =
            parse_ok(get_invite_link_handler(State(state), jar, Path(guild_id)).await).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Mengambil Invite Link");
    }

    #[tokio::test]
    async fn regenerate_invite_handler_success() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut mock = MockSocialService::new();

        mock.expect_regenerate_invite_service()
            .with(eq(guild_id), eq(user_id), eq("https://runetalk.app"))
            .times(1)
            .returning(|_, _, _| {
                Ok(InviteLinkResponse {
                    invite_code: "NEWCODE1".to_string(),
                    invite_link: "https://runetalk.app/invite/NEWCODE1".to_string(),
                })
            });

        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let (status, body) =
            parse_ok(regenerate_invite_handler(State(state), jar, Path(guild_id)).await).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.message, "Berhasil Memperbarui Invite Link");
    }

    #[tokio::test]
    async fn join_guild_handler_http_unauthorized() {
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let server = TestServer::new(social_test_router(state));

        let response = server.post("/guild/join/ABCD1234").await;
        response.assert_status_unauthorized();
    }
}
