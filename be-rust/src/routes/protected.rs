use crate::{
    app::AppState,
    middleware::auth::auth_middleware,
    modules::{
        graphql::handler::graphql_handler,
        socials::handler::{
            create_guild_handler, create_rift_handler, delete_guild_handler, delete_rift_handler,
            edit_guild_handler, edit_rift_handler, get_guild_handler, get_invite_link_handler,
            join_guild_handler, regenerate_invite_handler,
        },
        user::handler::{
            add_friend_accept_handler, add_friend_block_handler, add_friend_handler,
            add_friend_reject_handler, check_ally_handler, edit_user,
            list_incoming_requests_handler, profile_me_handler, profile_user_handler,
            remove_ally_handler,
        },
        sse::handler::{sse_friends_handler, sse_messages_handler},
        ws::handler::ws_handler,
    },
};
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/graphql", get(graphql_handler))
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
        .route("/user/{id}", get(profile_user_handler))
        .route("/ws", get(ws_handler))
        .route("/sse/friends", get(sse_friends_handler))
        .route("/sse/messages", get(sse_messages_handler))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
}
