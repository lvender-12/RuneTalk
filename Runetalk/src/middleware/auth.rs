use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::{
    app::AppState,
    errors::{AppResult, AuthError},
    utils::jwt::decode_jwt,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let token = jar
        .get("token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::Unauthorized)?;

    let claims = decode_jwt(&token, &state.config.jwt.secret)?;

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
