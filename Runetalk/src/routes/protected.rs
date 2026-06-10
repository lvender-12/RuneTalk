use crate::{
    app::AppState,
    middleware::auth::auth_middleware,
    modules::user::handler::{
        add_friend_accept_handler, add_friend_block_handler, add_friend_handler,
        add_friend_reject_handler, check_ally_handler, edit_user, list_incoming_requests_handler,
        profile_me_handler, profile_user_handler, remove_ally_handler,
    },
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
        .route("/user/requests", get(list_incoming_requests_handler))
        .route("/user/add/{username}", get(add_friend_handler))
        .route("/user/add/{id}/accept", get(add_friend_accept_handler))
        .route("/user/add/{id}/reject", get(add_friend_reject_handler))
        .route("/user/add/{id}/block", get(add_friend_block_handler))
        .route(
            "/user/ally/{id}",
            get(check_ally_handler).delete(remove_ally_handler),
        )
        .route("/user/{id}", get(profile_user_handler))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
}
