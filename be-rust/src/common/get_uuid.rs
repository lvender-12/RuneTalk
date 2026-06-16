use crate::{
    app::AppState,
    errors::{AppResult, AuthError},
    utils::jwt::get_uuid_from_token,
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

pub async fn current_user_id(jar: &CookieJar, state: &AppState) -> AppResult<Uuid> {
    let token = jar
        .get("token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::Unauthorized)?;

    Ok(get_uuid_from_token(&token, &state.config.jwt.secret)?.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_extra::extract::cookie::Cookie;
    use std::sync::Arc;

    use crate::{
        errors::AppError,
        modules::socials::service::MockSocialService,
        testing::{auth_cookie_jar, fixtures::dummy_config, test_app_state},
        utils::jwt::generate_jwt,
    };

    #[tokio::test]
    async fn current_user_id_returns_uuid_from_valid_token() {
        let user_id = Uuid::new_v4();
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let parsed = current_user_id(&jar, &state).await.unwrap();
        assert_eq!(parsed, user_id);
    }

    #[tokio::test]
    async fn current_user_id_unauthorized_without_cookie() {
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;

        let err = current_user_id(&CookieJar::new(), &state)
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Unauthorized)));
    }

    #[tokio::test]
    async fn current_user_id_invalid_token() {
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let jar = CookieJar::new().add(Cookie::new("token", "invalid.token.value"));

        let err = current_user_id(&jar, &state).await.unwrap_err();
        assert!(matches!(err, AppError::Jwt(_)));
    }

    #[test]
    fn generate_and_parse_user_id_from_config() {
        let config = dummy_config();
        let user_id = Uuid::new_v4();
        let token = generate_jwt(
            user_id.to_string(),
            "test@example.com".to_string(),
            &config,
        )
        .unwrap();

        let parsed: Uuid = get_uuid_from_token(&token, &config.jwt.secret)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(parsed, user_id);
    }
}
