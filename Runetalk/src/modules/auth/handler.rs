use crate::{
    app::AppState,
    modules::auth::dto::{RegisterDto, ResendOtpDto, VerifyOtpDto},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use tracing::debug;
use validator::Validate;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(err.to_string())).into_response();
    }

    debug!("payload : {:?}", payload);

    match state.auth_service.register_service(payload).await {
        Ok(api_response) => (StatusCode::CREATED, Json(api_response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn check_otp(
    State(state): State<AppState>,
    Json(payload): Json<VerifyOtpDto>,
) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(err.to_string())).into_response();
    }

    debug!("payload : {:?}", payload);

    match state.auth_service.verification_otp(payload).await {
        Ok(api_response) => (StatusCode::OK, Json(api_response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn resend_otp(
    State(state): State<AppState>,
    Json(payload): Json<ResendOtpDto>,
) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(err.to_string())).into_response();
    }

    debug!("payload : {:?}", payload);

    match state
        .auth_service
        .send_verification_otp(&payload.email)
        .await
    {
        Ok(api_response) => (StatusCode::OK, Json(api_response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}
