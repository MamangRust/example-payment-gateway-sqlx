use crate::{abstract_trait::session::SessionLimitMiddlewareTrait, domain::responses::Session};
use async_trait::async_trait;
use chrono::Duration;
use deadpool_redis::{Connection, Pool, redis::AsyncCommands};
use std::sync::Arc;
use tracing::{debug, error};

#[derive(Clone)]
pub struct SessionStore {
    pub pool: Arc<Pool>,
}

impl SessionStore {
    pub fn new(pool: Pool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    async fn get_conn(&self) -> Option<Connection> {
        match self.pool.get().await {
            Ok(conn) => Some(conn),
            Err(e) => {
                error!("Failed to get Redis connection from pool: {:?}", e);
                None
            }
        }
    }
}

#[async_trait]
impl SessionLimitMiddlewareTrait for SessionStore {
    async fn create_session(&self, session_id: &str, session: &Session, ttl: Duration) -> bool {
        let json_data = match serde_json::to_string(session) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize session: {:?}", e);
                return false;
            }
        };

        if let Some(mut conn) = self.get_conn().await {
            let result: Result<(), _> = conn
                .set_ex(session_id, &json_data, ttl.num_seconds() as u64)
                .await;

            match result {
                Ok(_) => {
                    debug!("Session created for session_id: {}", session_id);
                    true
                }
                Err(e) => {
                    error!("Failed to create session: {:?}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    async fn get_session(&self, session_id: &str) -> Option<Session> {
        let mut conn = self.get_conn().await?;
        let result: Result<Option<String>, _> = conn.get(session_id).await;

        match result {
            Ok(Some(data)) => match serde_json::from_str::<Session>(&data) {
                Ok(session) => {
                    debug!("Session retrieved for session_id: {}", session_id);
                    Some(session)
                }
                Err(e) => {
                    error!("Failed to deserialize session: {:?}", e);
                    None
                }
            },
            Ok(None) => {
                debug!("Session not found: {}", session_id);
                None
            }
            Err(e) => {
                error!("Redis get error for session {}: {:?}", session_id, e);
                None
            }
        }
    }

    async fn delete_session(&self, session_id: &str) -> bool {
        if let Some(mut conn) = self.get_conn().await {
            let result: Result<(), _> = conn.del(session_id).await;
            match result {
                Ok(_) => {
                    debug!("Session deleted: {}", session_id);
                    true
                }
                Err(e) => {
                    error!("Failed to delete session {}: {:?}", session_id, e);
                    false
                }
            }
        } else {
            false
        }
    }

    async fn refresh_session(&self, session_id: &str, ttl: Duration) -> bool {
        if let Some(mut conn) = self.get_conn().await {
            let result: Result<bool, _> = conn.expire(session_id, ttl.num_seconds()).await;
            match result {
                Ok(_) => {
                    debug!("Session TTL refreshed for session_id: {}", session_id);
                    true
                }
                Err(e) => {
                    error!("Failed to refresh session TTL {}: {:?}", session_id, e);
                    false
                }
            }
        } else {
            false
        }
    }
}
