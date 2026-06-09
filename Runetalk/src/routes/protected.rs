use crate::{
    app::AppState,
    middleware::auth::auth_middleware,
    modules::user::handler::{edit_user, profile_me_handler, profile_user_handler},
};
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch},
};

pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user/edit", patch(edit_user))
        .route("/user/me", get(profile_me_handler))
        .route("/user/{id}", get(profile_user_handler))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
}
