use crate::{app::AppState, middleware::auth::auth_middleware, modules::user::handler::edit_user};
use axum::{Router, middleware::from_fn_with_state, routing::patch};

pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user/edit", patch(edit_user))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
}
