use crate::{
    app::AppState,
    common::response::ApiResponse,
    errors::AppResult,
    modules::auth::dto::{LoginDto, RegisterDto, ResendOtpDto, VerifyOtpDto},
};
use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use http::StatusCode;
use tracing::debug;
use validator::Validate;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;

    debug!("payload : {:?}", payload);

    let _ = state.auth_service.register_service(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil mendaftar".to_string(),
        }),
    ))
}

pub async fn check_otp(
    State(state): State<AppState>,
    Json(payload): Json<VerifyOtpDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;

    debug!("payload : {:?}", payload);

    let _ = state.auth_service.verification_otp(payload).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil memverifikasi OTP".to_string(),
        }),
    ))
}

pub async fn resend_otp(
    State(state): State<AppState>,
    Json(payload): Json<ResendOtpDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;

    debug!("payload : {:?}", payload);

    let _ = state
        .auth_service
        .send_verification_otp(&payload.email)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil mengirim OTP".to_string(),
        }),
    ))
}

pub async fn login_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginDto>,
) -> AppResult<impl IntoResponse> {
    payload.validate()?;

    debug!("payload : {:?}", payload);

    let token = state.auth_service.login_service(payload).await?;

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let updated_jar = jar.add(cookie);

    Ok((
        StatusCode::OK,
        updated_jar,
        Json(ApiResponse {
            success: true,
            data: serde_json::json!({}),
            message: "Berhasil Login".to_string(),
        }),
    ))
}
