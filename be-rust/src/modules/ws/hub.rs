use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::modules::ws::dto::WsServerMessage;

const CHANNEL_CAPACITY: usize = 256;

#[derive(Clone)]
pub struct WsHub {
    inner: Arc<WsHubInner>,
}

struct WsHubInner {
    rift_subscribers: RwLock<HashMap<Uuid, HashSet<Uuid>>>,
    scroll_subscribers: RwLock<HashMap<Uuid, HashSet<Uuid>>>,
    connections: RwLock<HashMap<Uuid, mpsc::Sender<String>>>,
    connection_users: RwLock<HashMap<Uuid, Uuid>>,
    user_connections: RwLock<HashMap<Uuid, HashSet<Uuid>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(WsHubInner {
                rift_subscribers: RwLock::new(HashMap::new()),
                scroll_subscribers: RwLock::new(HashMap::new()),
                connections: RwLock::new(HashMap::new()),
                connection_users: RwLock::new(HashMap::new()),
                user_connections: RwLock::new(HashMap::new()),
            }),
        }
    }

    pub async fn register(&self, user_id: Uuid) -> (Uuid, mpsc::Receiver<String>, bool) {
        let conn_id = Uuid::new_v4();
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);

        self.inner.connections.write().await.insert(conn_id, tx);
        self.inner
            .connection_users
            .write()
            .await
            .insert(conn_id, user_id);

        let mut user_connections = self.inner.user_connections.write().await;
        let connections = user_connections.entry(user_id).or_default();
        let is_first_connection = connections.is_empty();
        connections.insert(conn_id);

        (conn_id, rx, is_first_connection)
    }

    pub async fn unregister(&self, conn_id: Uuid) -> Option<(Uuid, bool)> {
        self.inner.connections.write().await.remove(&conn_id);

        let user_id = self
            .inner
            .connection_users
            .write()
            .await
            .remove(&conn_id)?;

        let mut rift_subs = self.inner.rift_subscribers.write().await;
        for subscribers in rift_subs.values_mut() {
            subscribers.remove(&conn_id);
        }
        rift_subs.retain(|_, subscribers| !subscribers.is_empty());

        let mut scroll_subs = self.inner.scroll_subscribers.write().await;
        for subscribers in scroll_subs.values_mut() {
            subscribers.remove(&conn_id);
        }
        scroll_subs.retain(|_, subscribers| !subscribers.is_empty());

        let mut user_connections = self.inner.user_connections.write().await;
        let is_last_connection = if let Some(connections) = user_connections.get_mut(&user_id) {
            connections.remove(&conn_id);
            let empty = connections.is_empty();
            if empty {
                user_connections.remove(&user_id);
            }
            empty
        } else {
            true
        };

        Some((user_id, is_last_connection))
    }

    pub async fn online_user_ids(&self) -> Vec<Uuid> {
        self.inner
            .user_connections
            .read()
            .await
            .keys()
            .copied()
            .collect()
    }

    pub async fn subscribe_rift(&self, conn_id: Uuid, rift_id: Uuid) {
        self.inner
            .rift_subscribers
            .write()
            .await
            .entry(rift_id)
            .or_default()
            .insert(conn_id);
    }

    pub async fn unsubscribe_rift(&self, conn_id: Uuid, rift_id: Uuid) {
        let mut rift_subs = self.inner.rift_subscribers.write().await;
        if let Some(subscribers) = rift_subs.get_mut(&rift_id) {
            subscribers.remove(&conn_id);
            if subscribers.is_empty() {
                rift_subs.remove(&rift_id);
            }
        }
    }

    pub async fn subscribe_scroll(&self, conn_id: Uuid, scroll_id: Uuid) {
        self.inner
            .scroll_subscribers
            .write()
            .await
            .entry(scroll_id)
            .or_default()
            .insert(conn_id);
    }

    pub async fn unsubscribe_scroll(&self, conn_id: Uuid, scroll_id: Uuid) {
        let mut scroll_subs = self.inner.scroll_subscribers.write().await;
        if let Some(subscribers) = scroll_subs.get_mut(&scroll_id) {
            subscribers.remove(&conn_id);
            if subscribers.is_empty() {
                scroll_subs.remove(&scroll_id);
            }
        }
    }

    pub async fn broadcast_rift(&self, rift_id: Uuid, message: &WsServerMessage) {
        self.broadcast_room(&self.inner.rift_subscribers, rift_id, message)
            .await;
    }

    pub async fn broadcast_scroll(&self, scroll_id: Uuid, message: &WsServerMessage) {
        self.broadcast_room(&self.inner.scroll_subscribers, scroll_id, message)
            .await;
    }

    pub async fn broadcast_all(&self, message: &WsServerMessage) {
        let payload = match serde_json::to_string(message) {
            Ok(payload) => payload,
            Err(_) => return,
        };

        let connections = self.inner.connections.read().await;
        for tx in connections.values() {
            let _ = tx.try_send(payload.clone());
        }
    }

    pub async fn send_to_connection(&self, conn_id: Uuid, message: &WsServerMessage) {
        let payload = match serde_json::to_string(message) {
            Ok(payload) => payload,
            Err(_) => return,
        };

        let connections = self.inner.connections.read().await;
        if let Some(tx) = connections.get(&conn_id) {
            let _ = tx.try_send(payload);
        }
    }

    async fn broadcast_room(
        &self,
        room: &RwLock<HashMap<Uuid, HashSet<Uuid>>>,
        room_id: Uuid,
        message: &WsServerMessage,
    ) {
        let payload = match serde_json::to_string(message) {
            Ok(payload) => payload,
            Err(_) => return,
        };

        let subscribers = room.read().await.get(&room_id).cloned().unwrap_or_default();
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

impl Default for WsHub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entity::PresenceStatus,
        modules::ws::dto::WsServerMessage,
    };

    #[tokio::test]
    async fn register_marks_first_connection() {
        let hub = WsHub::new();
        let user_id = Uuid::new_v4();

        let (_, _, is_first) = hub.register(user_id).await;
        assert!(is_first);

        let (_, _, is_first) = hub.register(user_id).await;
        assert!(!is_first);
    }

    #[tokio::test]
    async fn broadcast_rift_reaches_subscribed_connection() {
        let hub = WsHub::new();
        let user_id = Uuid::new_v4();
        let rift_id = Uuid::new_v4();
        let (conn_id, mut rx, _) = hub.register(user_id).await;
        hub.subscribe_rift(conn_id, rift_id).await;

        hub.broadcast_rift(
            rift_id,
            &WsServerMessage::SubscribedRift { rift_id },
        )
        .await;

        let payload = rx.recv().await.expect("rift broadcast payload");
        let message: WsServerMessage = serde_json::from_str(&payload).expect("rift message json");
        assert!(matches!(message, WsServerMessage::SubscribedRift { .. }));
    }

    #[tokio::test]
    async fn broadcast_scroll_reaches_subscribed_connection() {
        let hub = WsHub::new();
        let user_id = Uuid::new_v4();
        let scroll_id = Uuid::new_v4();
        let (conn_id, mut rx, _) = hub.register(user_id).await;
        hub.subscribe_scroll(conn_id, scroll_id).await;

        hub.broadcast_scroll(
            scroll_id,
            &WsServerMessage::SubscribedScroll { scroll_id },
        )
        .await;

        let payload = rx.recv().await.expect("scroll broadcast payload");
        let message: WsServerMessage = serde_json::from_str(&payload).expect("scroll message json");
        assert!(matches!(message, WsServerMessage::SubscribedScroll { .. }));
    }

    #[tokio::test]
    async fn send_to_connection_targets_single_client() {
        let hub = WsHub::new();
        let user_id = Uuid::new_v4();
        let (conn_id, mut rx, _) = hub.register(user_id).await;
        let (_, mut other_rx, _) = hub.register(user_id).await;

        hub.send_to_connection(
            conn_id,
            &WsServerMessage::PresenceUpdate {
                user_id,
                status: PresenceStatus::Online,
            },
        )
        .await;

        rx.recv().await.expect("direct message");
        assert!(other_rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn broadcast_all_reaches_every_connection() {
        let hub = WsHub::new();
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();
        let (_, mut rx_a, _) = hub.register(user_a).await;
        let (_, mut rx_b, _) = hub.register(user_b).await;

        hub.broadcast_all(&WsServerMessage::PresenceSnapshot {
            online_users: vec![user_a, user_b],
        })
        .await;

        rx_a.recv().await.expect("broadcast to a");
        rx_b.recv().await.expect("broadcast to b");
    }

    #[tokio::test]
    async fn unregister_cleans_up_subscriptions_and_tracks_last_connection() {
        let hub = WsHub::new();
        let user_id = Uuid::new_v4();
        let rift_id = Uuid::new_v4();
        let (conn_a, mut rx_a, _) = hub.register(user_id).await;
        let (conn_b, _, _) = hub.register(user_id).await;
        hub.subscribe_rift(conn_a, rift_id).await;

        let (disconnected_user, is_last) = hub.unregister(conn_b).await.expect("unregister b");
        assert_eq!(disconnected_user, user_id);
        assert!(!is_last);

        hub.broadcast_rift(
            rift_id,
            &WsServerMessage::SubscribedRift { rift_id },
        )
        .await;
        rx_a.recv().await.expect("still subscribed on conn a");

        let (disconnected_user, is_last) = hub.unregister(conn_a).await.expect("unregister a");
        assert_eq!(disconnected_user, user_id);
        assert!(is_last);
        assert!(hub.online_user_ids().await.is_empty());
    }
}
