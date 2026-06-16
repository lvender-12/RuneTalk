use crate::{
    app::AppState,
    modules::auth::handler::{
        check_otp, login_handler, logout_handler, register_handler, resend_otp,
    },
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/resend", post(resend_otp))
        .route("/auth/verify", post(check_otp))
        .route("/auth/login", post(login_handler))
        .route("/auth/logout", get(logout_handler))
}
