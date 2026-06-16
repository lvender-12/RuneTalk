use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use futures::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::{
    app::AppState,
    common::get_uuid::current_user_id,
    entity::PresenceStatus,
    errors::AppResult,
    modules::{
        sse::dto::SseMessageEvent,
        ws::dto::{WsClientMessage, WsServerMessage},
    },
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, user_id)))
}

async fn handle_socket(socket: WebSocket, state: AppState, user_id: Uuid) {
    let (conn_id, mut hub_rx, is_first_connection) = state.ws_hub.register(user_id).await;

    if is_first_connection {
        mark_presence(&state, user_id, PresenceStatus::Online).await;
    }

    send_direct(
        &state,
        conn_id,
        WsServerMessage::PresenceSnapshot {
            online_users: state.ws_hub.online_user_ids().await,
        },
    )
    .await;

    let (mut sender, mut receiver) = socket.split();

    loop {
        tokio::select! {
            maybe_msg = receiver.next() => {
                match maybe_msg {
                    Some(Ok(Message::Text(text))) => {
                        handle_client_message(&state, conn_id, user_id, &text).await;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            maybe_out = hub_rx.recv() => {
                match maybe_out {
                    Some(payload) => {
                        if sender.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
    }

    if let Some((disconnected_user, is_last_connection)) = state.ws_hub.unregister(conn_id).await {
        if is_last_connection {
            mark_presence(&state, disconnected_user, PresenceStatus::Offline).await;
        }
    }
}

async fn mark_presence(state: &AppState, user_id: Uuid, status: PresenceStatus) {
    if state.ws_service.set_presence(user_id, status).await.is_ok() {
        state
            .ws_hub
            .broadcast_all(&WsServerMessage::PresenceUpdate {
                user_id,
                status,
            })
            .await;
    }
}

async fn handle_client_message(state: &AppState, conn_id: Uuid, user_id: Uuid, text: &str) {
    let message: WsClientMessage = match serde_json::from_str(text) {
        Ok(message) => message,
        Err(err) => {
            send_direct(state, conn_id, WsServerMessage::error(err.to_string())).await;
            return;
        }
    };

    match message {
        WsClientMessage::SubscribeRift { rift_id } => {
            match state.ws_service.verify_rift_access(rift_id, user_id).await {
                Ok(()) => {
                    state.ws_hub.subscribe_rift(conn_id, rift_id).await;
                    send_direct(
                        state,
                        conn_id,
                        WsServerMessage::SubscribedRift { rift_id },
                    )
                    .await;
                }
                Err(err) => send_direct(state, conn_id, WsServerMessage::error(err.to_string())).await,
            }
        }
        WsClientMessage::UnsubscribeRift { rift_id } => {
            state.ws_hub.unsubscribe_rift(conn_id, rift_id).await;
            send_direct(
                state,
                conn_id,
                WsServerMessage::UnsubscribedRift { rift_id },
            )
            .await;
        }
        WsClientMessage::SubscribeScroll { scroll_id } => {
            match state
                .ws_service
                .verify_scroll_access(scroll_id, user_id)
                .await
            {
                Ok(()) => {
                    state.ws_hub.subscribe_scroll(conn_id, scroll_id).await;
                    send_direct(
                        state,
                        conn_id,
                        WsServerMessage::SubscribedScroll { scroll_id },
                    )
                    .await;
                }
                Err(err) => send_direct(state, conn_id, WsServerMessage::error(err.to_string())).await,
            }
        }
        WsClientMessage::UnsubscribeScroll { scroll_id } => {
            state.ws_hub.unsubscribe_scroll(conn_id, scroll_id).await;
            send_direct(
                state,
                conn_id,
                WsServerMessage::UnsubscribedScroll { scroll_id },
            )
            .await;
        }
        WsClientMessage::SendEcho { payload } => {
            match state.ws_service.send_echo_service(payload, user_id).await {
                Ok(echo) => {
                    state
                        .ws_hub
                        .broadcast_rift(echo.rift_id, &WsServerMessage::Echo { data: echo })
                        .await;
                }
                Err(err) => send_direct(state, conn_id, WsServerMessage::error(err.to_string())).await,
            }
        }
        WsClientMessage::SendWhisper { payload } => {
            match state.ws_service.send_whisper_service(payload, user_id).await {
                Ok(whisper) => {
                    state
                        .ws_hub
                        .broadcast_scroll(
                            whisper.scroll_id,
                            &WsServerMessage::Whisper {
                                data: whisper.clone(),
                            },
                        )
                        .await;

                    if let Ok(recipient_id) = state
                        .ws_service
                        .scroll_recipient_id(whisper.scroll_id, user_id)
                        .await
                    {
                        state
                            .sse_hub
                            .send_message_event(
                                recipient_id,
                                &SseMessageEvent::WhisperReceived { data: whisper },
                            )
                            .await;
                    }
                }
                Err(err) => send_direct(state, conn_id, WsServerMessage::error(err.to_string())).await,
            }
        }
    }
}

async fn send_direct(state: &AppState, conn_id: Uuid, message: WsServerMessage) {
    state.ws_hub.send_to_connection(conn_id, &message).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_extra::extract::cookie::Cookie;
    use axum_test::TestServer;
    use mockall::predicate::*;
    use std::sync::Arc;

    use crate::{
        modules::ws::{dto::WsServerMessage, service::MockWsService},
        testing::{
            auth_token,
            fixtures::{dummy_echo, dummy_presence, dummy_rift},
            router::realtime_test_router,
            test_ws_app_state,
        },
    };

    fn ws_test_server(state: crate::app::AppState) -> TestServer {
        TestServer::builder()
            .http_transport()
            .build(realtime_test_router(state))
    }

    #[tokio::test]
    async fn ws_handler_unauthorized_without_cookie() {
        let mock = MockWsService::new();
        let state = test_ws_app_state(Arc::new(mock)).await;
        let server = ws_test_server(state);

        server
            .get_websocket("/ws")
            .await
            .assert_status(http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn ws_handler_sends_presence_snapshot_on_connect() {
        let user_id = Uuid::new_v4();
        let mut mock = MockWsService::new();

        mock.expect_set_presence()
            .returning(move |uid, status| Ok(dummy_presence(uid, status)));

        let state = test_ws_app_state(Arc::new(mock)).await;
        let token = auth_token(user_id, &state.config);
        let server = ws_test_server(state);

        let mut websocket = server
            .get_websocket("/ws")
            .add_cookie(Cookie::new("token", token))
            .await
            .into_websocket()
            .await;

        let message: WsServerMessage = websocket.receive_json().await;
        assert!(matches!(message, WsServerMessage::PresenceSnapshot { .. }));
    }

    #[tokio::test]
    async fn ws_handler_subscribe_rift_success() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let rift_id = rift.id;
        let mut mock = MockWsService::new();

        mock.expect_set_presence()
            .returning(move |uid, status| Ok(dummy_presence(uid, status)));

        mock.expect_verify_rift_access()
            .with(eq(rift_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(()));

        let state = test_ws_app_state(Arc::new(mock)).await;
        let token = auth_token(user_id, &state.config);
        let server = ws_test_server(state);

        let mut websocket = server
            .get_websocket("/ws")
            .add_cookie(Cookie::new("token", token))
            .await
            .into_websocket()
            .await;

        let _: WsServerMessage = websocket.receive_json().await;

        websocket
            .send_json(&serde_json::json!({
                "type": "subscribe_rift",
                "rift_id": rift_id,
            }))
            .await;

        let message: WsServerMessage = websocket.receive_json().await;
        assert!(matches!(
            message,
            WsServerMessage::SubscribedRift { rift_id: id } if id == rift_id
        ));
    }

    #[tokio::test]
    async fn ws_handler_send_echo_broadcasts_to_rift() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let rift_id = rift.id;
        let echo = dummy_echo(rift_id, user_id);
        let mut mock = MockWsService::new();

        mock.expect_set_presence()
            .returning(move |uid, status| Ok(dummy_presence(uid, status)));

        mock.expect_verify_rift_access()
            .with(eq(rift_id), eq(user_id))
            .returning(|_, _| Ok(()));

        mock.expect_send_echo_service()
            .times(1)
            .returning(move |_, _| Ok(echo.clone()));

        let state = test_ws_app_state(Arc::new(mock)).await;
        let token = auth_token(user_id, &state.config);
        let server = ws_test_server(state);

        let mut websocket = server
            .get_websocket("/ws")
            .add_cookie(Cookie::new("token", token))
            .await
            .into_websocket()
            .await;

        let _: WsServerMessage = websocket.receive_json().await;

        websocket
            .send_json(&serde_json::json!({
                "type": "subscribe_rift",
                "rift_id": rift_id,
            }))
            .await;
        let _: WsServerMessage = websocket.receive_json().await;

        websocket
            .send_json(&serde_json::json!({
                "type": "send_echo",
                "rift_id": rift_id,
                "content": "hello echo",
            }))
            .await;

        let message: WsServerMessage = websocket.receive_json().await;
        assert!(matches!(message, WsServerMessage::Echo { .. }));
    }

    #[tokio::test]
    async fn ws_handler_invalid_message_returns_error() {
        let user_id = Uuid::new_v4();
        let mut mock = MockWsService::new();

        mock.expect_set_presence()
            .returning(move |uid, status| Ok(dummy_presence(uid, status)));

        let state = test_ws_app_state(Arc::new(mock)).await;
        let token = auth_token(user_id, &state.config);
        let server = ws_test_server(state);

        let mut websocket = server
            .get_websocket("/ws")
            .add_cookie(Cookie::new("token", token))
            .await
            .into_websocket()
            .await;

        let _: WsServerMessage = websocket.receive_json().await;
        websocket.send_text("{not-json").await;

        let message: WsServerMessage = websocket.receive_json().await;
        assert!(matches!(message, WsServerMessage::Error { .. }));
    }
}
