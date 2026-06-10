use axum::{
    Json,
    extract::{Multipart, Path, State},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use http::StatusCode;
use tracing::debug;
use uuid::Uuid;

use crate::{
    app::AppState,
    common::response::ApiResponse,
    errors::{AppResult, AuthError},
    utils::jwt::get_uuid_from_token,
};

async fn current_user_id(jar: &CookieJar, state: &AppState) -> AppResult<Uuid> {
    let token = jar
        .get("token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::Unauthorized)?;

    Ok(get_uuid_from_token(&token, &state.config.jwt.secret)?.parse()?)
}

pub async fn edit_user(
    State(state): State<AppState>,
    jar: CookieJar,
    multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    debug!("{:?}", multipart);
    let uuid = current_user_id(&jar, &state).await?;

    debug!("{}", uuid);
    let user = state
        .user_service
        .edit_user_service(&state, multipart, uuid)
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

pub async fn profile_me_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let uuid = current_user_id(&jar, &state).await?;
    let profile = state.user_service.profile_user(uuid).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!(profile),
            message: "Berhasil Mengambil Profil".to_string(),
        }),
    ))
}

pub async fn profile_user_handler(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> AppResult<impl IntoResponse> {
    let profile = state.user_service.profile_user(id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!(profile),
            message: "Berhasil Mengambil Profil".to_string(),
        }),
    ))
}

pub async fn add_friend_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(username): Path<String>,
) -> AppResult<impl IntoResponse> {
    let from = current_user_id(&jar, &state).await?;

    state
        .user_service
        .add_friend_service(&username, from)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil Mengirim Permintaan".to_string(),
        }),
    ))
}

pub async fn list_incoming_requests_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    let requests = state
        .user_service
        .list_incoming_requests_service(user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!(requests),
            message: "Berhasil Mengambil Permintaan Pertemanan".to_string(),
        }),
    ))
}

pub async fn add_friend_accept_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let to = current_user_id(&jar, &state).await?;

    state.user_service.accept_friend_service(id, to).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil Menerima Permintaan".to_string(),
        }),
    ))
}

pub async fn add_friend_reject_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let to = current_user_id(&jar, &state).await?;

    state.user_service.reject_friend_service(id, to).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil Menolak Permintaan".to_string(),
        }),
    ))
}

pub async fn add_friend_block_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let blocker = current_user_id(&jar, &state).await?;

    state.user_service.block_friend_service(blocker, id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil Memblokir Pengguna".to_string(),
        }),
    ))
}

pub async fn check_ally_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(other_user_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let current_user = current_user_id(&jar, &state).await?;

    let is_ally = state
        .user_service
        .is_ally_service(current_user, other_user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({ "is_ally": is_ally }),
            message: "Status pertemanan berhasil dicek".to_string(),
        }),
    ))
}

pub async fn remove_ally_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(friend_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let current_user = current_user_id(&jar, &state).await?;

    state
        .user_service
        .remove_ally_service(current_user, friend_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil menghapus teman".to_string(),
        }),
    ))
}
