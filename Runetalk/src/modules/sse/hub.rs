use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::modules::sse::dto::{SseFriendEvent, SseMessageEvent};

const CHANNEL_CAPACITY: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SseStream {
    Friends,
    Messages,
}

#[derive(Clone)]
pub struct SseHub {
    inner: Arc<SseHubInner>,
}

struct SseHubInner {
    friend_subscribers: RwLock<HashMap<Uuid, HashSet<Uuid>>>,
    message_subscribers: RwLock<HashMap<Uuid, HashSet<Uuid>>>,
    connections: RwLock<HashMap<Uuid, mpsc::Sender<String>>>,
}

impl SseHub {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SseHubInner {
                friend_subscribers: RwLock::new(HashMap::new()),
                message_subscribers: RwLock::new(HashMap::new()),
                connections: RwLock::new(HashMap::new()),
            }),
        }
    }

    pub async fn register(&self) -> (Uuid, mpsc::Receiver<String>) {
        let conn_id = Uuid::new_v4();
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        self.inner.connections.write().await.insert(conn_id, tx);
        (conn_id, rx)
    }

    pub async fn unregister(&self, conn_id: Uuid) {
        self.inner.connections.write().await.remove(&conn_id);

        let mut friend_subs = self.inner.friend_subscribers.write().await;
        for subscribers in friend_subs.values_mut() {
            subscribers.remove(&conn_id);
        }
        friend_subs.retain(|_, subscribers| !subscribers.is_empty());

        let mut message_subs = self.inner.message_subscribers.write().await;
        for subscribers in message_subs.values_mut() {
            subscribers.remove(&conn_id);
        }
        message_subs.retain(|_, subscribers| !subscribers.is_empty());
    }

    pub async fn subscribe_user(&self, conn_id: Uuid, user_id: Uuid, stream: SseStream) {
        let room = match stream {
            SseStream::Friends => &self.inner.friend_subscribers,
            SseStream::Messages => &self.inner.message_subscribers,
        };

        room.write()
            .await
            .entry(user_id)
            .or_default()
            .insert(conn_id);
    }

    pub async fn send_friend_event(&self, user_id: Uuid, event: &SseFriendEvent) {
        self.send_to_user(&self.inner.friend_subscribers, user_id, event)
            .await;
    }

    pub async fn send_message_event(&self, user_id: Uuid, event: &SseMessageEvent) {
        self.send_to_user(&self.inner.message_subscribers, user_id, event)
            .await;
    }

    async fn send_to_user<T: serde::Serialize>(
        &self,
        room: &RwLock<HashMap<Uuid, HashSet<Uuid>>>,
        user_id: Uuid,
        event: &T,
    ) {
        let payload = match serde_json::to_string(event) {
            Ok(payload) => payload,
            Err(_) => return,
        };

        let subscribers = room.read().await.get(&user_id).cloned().unwrap_or_default();
        if subscribers.is_empty() {
            return;
        }

        let connections = self.inner.connections.read().await;
        for conn_id in subscribers {
            if let Some(tx) = connections.get(&conn_id) {
                let _ = tx.try_send(payload.clone());
            }
        }
    }
}

impl Default for SseHub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        modules::sse::dto::{SseFriendEvent, SseMessageEvent},
        testing::fixtures::{dummy_echo, dummy_friend_request, dummy_whisper},
    };

    #[tokio::test]
    async fn send_friend_event_delivers_to_subscriber() {
        let hub = SseHub::new();
        let user_id = Uuid::new_v4();
        let (conn_id, mut rx) = hub.register().await;
        hub.subscribe_user(conn_id, user_id, SseStream::Friends).await;

        let request = dummy_friend_request(Uuid::new_v4());
        hub.send_friend_event(
            user_id,
            &SseFriendEvent::FriendRequestReceived {
                data: request.clone(),
            },
        )
        .await;

        let payload = rx.recv().await.expect("friend event payload");
        let event: SseFriendEvent = serde_json::from_str(&payload).expect("friend event json");
        assert!(matches!(
            event,
            SseFriendEvent::FriendRequestReceived { .. }
        ));
    }

    #[tokio::test]
    async fn friend_events_do_not_reach_message_subscribers() {
        let hub = SseHub::new();
        let user_id = Uuid::new_v4();
        let (conn_id, mut rx) = hub.register().await;
        hub.subscribe_user(conn_id, user_id, SseStream::Messages)
            .await;

        hub.send_friend_event(
            user_id,
            &SseFriendEvent::FriendRemoved { user_id: Uuid::new_v4() },
        )
        .await;

        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn send_message_event_delivers_to_subscriber() {
        let hub = SseHub::new();
        let user_id = Uuid::new_v4();
        let scroll_id = Uuid::new_v4();
        let (conn_id, mut rx) = hub.register().await;
        hub.subscribe_user(conn_id, user_id, SseStream::Messages)
            .await;

        let whisper = dummy_whisper(scroll_id, user_id);
        hub.send_message_event(
            user_id,
            &SseMessageEvent::WhisperReceived {
                data: whisper.clone(),
            },
        )
        .await;

        let payload = rx.recv().await.expect("message event payload");
        let event: SseMessageEvent = serde_json::from_str(&payload).expect("message event json");
        assert!(matches!(event, SseMessageEvent::WhisperReceived { .. }));
    }

    #[tokio::test]
    async fn unregister_stops_event_delivery() {
        let hub = SseHub::new();
        let user_id = Uuid::new_v4();
        let (conn_id, mut rx) = hub.register().await;
        hub.subscribe_user(conn_id, user_id, SseStream::Messages)
            .await;
        hub.unregister(conn_id).await;

        let echo = dummy_echo(Uuid::new_v4(), user_id);
        hub.send_message_event(
            user_id,
            &SseMessageEvent::EchoReceived { data: echo },
        )
        .await;

        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn multiple_subscribers_receive_same_event() {
        let hub = SseHub::new();
        let user_id = Uuid::new_v4();
        let (conn_a, mut rx_a) = hub.register().await;
        let (conn_b, mut rx_b) = hub.register().await;
        hub.subscribe_user(conn_a, user_id, SseStream::Friends).await;
        hub.subscribe_user(conn_b, user_id, SseStream::Friends).await;

        hub.send_friend_event(
            user_id,
            &SseFriendEvent::FriendRequestRejected { user_id },
        )
        .await;

        rx_a.recv().await.expect("subscriber a payload");
        rx_b.recv().await.expect("subscriber b payload");
    }
}
