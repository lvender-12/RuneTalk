use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

use crate::{
    app::AppState,
    middleware::auth::auth_middleware,
    modules::{
        socials::handler::{
            create_guild_handler, create_rift_handler, delete_guild_handler, delete_rift_handler,
            edit_guild_handler, edit_rift_handler, get_guild_handler, get_invite_link_handler,
            join_guild_handler, regenerate_invite_handler,
        },
        sse::handler::{sse_friends_handler, sse_messages_handler},
        ws::handler::ws_handler,
    },
};

pub fn social_test_router(state: AppState) -> Router {
    Router::new()
        .route("/guild", post(create_guild_handler))
        .route("/guild/join/{invite_code}", post(join_guild_handler))
        .route(
            "/guild/{id}",
            get(get_guild_handler)
                .patch(edit_guild_handler)
                .delete(delete_guild_handler),
        )
        .route(
            "/guild/{id}/invite",
            get(get_invite_link_handler).post(regenerate_invite_handler),
        )
        .route("/guild/{guild_id}/rift", post(create_rift_handler))
        .route(
            "/guild/{guild_id}/rift/{rift_id}",
            patch(edit_rift_handler).delete(delete_rift_handler),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

pub fn realtime_test_router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/sse/friends", get(sse_friends_handler))
        .route("/sse/messages", get(sse_messages_handler))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
