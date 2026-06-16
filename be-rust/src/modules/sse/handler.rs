use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use axum_extra::extract::CookieJar;
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    app::AppState,
    common::get_uuid::current_user_id,
    errors::AppResult,
    modules::sse::hub::SseStream,
};

pub async fn sse_friends_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    Ok(build_sse_response(state, user_id, SseStream::Friends))
}

pub async fn sse_messages_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let user_id = current_user_id(&jar, &state).await?;
    Ok(build_sse_response(state, user_id, SseStream::Messages))
}

fn build_sse_response(
    state: AppState,
    user_id: Uuid,
    stream: SseStream,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let connected_event = Event::default()
        .event("connected")
        .data(serde_json::json!({ "user_id": user_id }).to_string());

    let sse_stream = futures::stream::once(async move { Ok(connected_event) }).chain(
        futures::stream::unfold(
            SseConnectionState {
                state,
                user_id,
                stream,
                conn_id: None,
                rx: None,
            },
            |mut conn| async move {
                if conn.rx.is_none() {
                    let (conn_id, rx) = conn.state.sse_hub.register().await;
                    conn.state
                        .sse_hub
                        .subscribe_user(conn_id, conn.user_id, conn.stream)
                        .await;
                    conn.conn_id = Some(conn_id);
                    conn.rx = Some(rx);
                }

                let rx = conn.rx.as_mut()?;
                match rx.recv().await {
                    Some(payload) => Some((
                        Ok(Event::default().data(payload)),
                        conn,
                    )),
                    None => {
                        if let Some(conn_id) = conn.conn_id {
                            conn.state.sse_hub.unregister(conn_id).await;
                        }
                        None
                    }
                }
            },
        ),
    );

    Sse::new(sse_stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}

struct SseConnectionState {
    state: AppState,
    user_id: Uuid,
    stream: SseStream,
    conn_id: Option<Uuid>,
    rx: Option<mpsc::Receiver<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_extra::extract::CookieJar;
    use http::{HeaderValue, StatusCode};
    use std::{sync::Arc, time::Duration};

    use crate::{
        errors::{AppError, AuthError},
        modules::{
            socials::service::MockSocialService,
            sse::dto::SseFriendEvent,
        },
        testing::{auth_cookie_jar, fixtures::dummy_friend_request, test_app_state},
    };

    async fn first_sse_chunk(response: axum::response::Response) -> String {
        use axum::body::Body;
        use futures::StreamExt;

        let body = response.into_body();
        let chunk = tokio::time::timeout(Duration::from_secs(1), async {
            let mut stream = Body::into_data_stream(body);
            stream.next().await
        })
        .await
        .expect("timeout waiting for sse chunk")
        .expect("sse stream ended")
        .expect("sse chunk bytes");

        String::from_utf8(chunk.to_vec()).expect("utf8 sse chunk")
    }

    #[tokio::test]
    async fn sse_friends_handler_unauthorized_without_cookie() {
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;

        match sse_friends_handler(State(state), CookieJar::new()).await {
            Err(err) => assert!(matches!(err, AppError::Auth(AuthError::Unauthorized))),
            Ok(_) => panic!("expected unauthorized error"),
        }
    }

    #[tokio::test]
    async fn sse_friends_handler_returns_connected_event() {
        let user_id = Uuid::new_v4();
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let response = sse_friends_handler(State(state.clone()), jar)
            .await
            .expect("handler should succeed")
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/event-stream"))
        );

        let chunk = first_sse_chunk(response).await;
        assert!(chunk.contains("event: connected"));
        assert!(chunk.contains(&user_id.to_string()));
    }

    #[tokio::test]
    async fn sse_friends_handler_delivers_hub_events() {
        use axum::body::Body;
        use futures::StreamExt;

        let user_id = Uuid::new_v4();
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let response = sse_friends_handler(State(state.clone()), jar)
            .await
            .expect("handler should succeed")
            .into_response();

        let mut body = Body::into_data_stream(response.into_body());
        let connected = tokio::time::timeout(Duration::from_secs(1), body.next())
            .await
            .expect("timeout waiting for connected event")
            .expect("sse stream ended")
            .expect("connected chunk bytes");
        let connected_text = String::from_utf8(connected.to_vec()).expect("utf8 connected chunk");
        assert!(connected_text.contains("event: connected"));

        let request = dummy_friend_request(Uuid::new_v4());
        state
            .sse_hub
            .send_friend_event(
                user_id,
                &SseFriendEvent::FriendRequestReceived { data: request },
            )
            .await;

        let event_chunk = tokio::time::timeout(Duration::from_secs(1), body.next())
            .await
            .expect("timeout waiting for friend event")
            .expect("sse stream ended")
            .expect("friend event chunk bytes");
        let event_text = String::from_utf8(event_chunk.to_vec()).expect("utf8 friend event chunk");
        assert!(event_text.contains("friend_request_received"));
    }

    #[tokio::test]
    async fn sse_messages_handler_returns_connected_event() {
        let user_id = Uuid::new_v4();
        let mock = MockSocialService::new();
        let state = test_app_state(Arc::new(mock)).await;
        let jar = auth_cookie_jar(user_id, &state.config);

        let response = sse_messages_handler(State(state), jar)
            .await
            .expect("handler should succeed")
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        let chunk = first_sse_chunk(response).await;
        assert!(chunk.contains("event: connected"));
    }
}
