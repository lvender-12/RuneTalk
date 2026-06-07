use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use http::HeaderMap;

use crate::{
    app::AppState,
    errors::{AppResult, AuthError},
};

pub async fn api_keys_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> AppResult<Response> {
    let api_key = &state.config.api.secret;

    let secret = headers
        .get("X-API-SECRET")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if secret != api_key {
        return Err(AuthError::InvalidApiKey.into());
    }

    Ok(next.run(req).await)
}
