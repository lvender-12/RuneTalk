use axum::{
    Json,
    extract::{Multipart, State},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use http::StatusCode;
use tracing::debug;

use crate::{
    app::AppState,
    common::response::ApiResponse,
    errors::{AppResult, AuthError},
    utils::jwt::get_uuid_from_token,
};

pub async fn edit_user(
    State(state): State<AppState>,
    jar: CookieJar,
    multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    debug!("{:?}", multipart);
    let token = jar
        .get("token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::Unauthorized)?;

    let uuid = get_uuid_from_token(&token, &state.config.jwt.secret)?;

    debug!("{}", uuid);
    let user = state
        .user_service
        .edit_user_service(&state, multipart, uuid.parse()?)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!(user),
            message: "Berhasil Mengubah Data".to_string(),
        }),
    ))
}
