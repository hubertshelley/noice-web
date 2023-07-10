use std::sync::Arc;
use async_session::{MemoryStore, SessionStore};
use silent::{MiddleWareHandler, Request, Response, Result, SilentError, StatusCode};
use async_trait::async_trait;
use sqlx::MySqlPool;
use tokio::sync::{RwLock};
use crate::models::{User, UserAuth};


pub struct AuthorisationMiddleware {
    pub user: UserAuth,
}

impl Default for AuthorisationMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorisationMiddleware {
    pub fn new() -> Self {
        let user = UserAuth::AnyOneUser;
        AuthorisationMiddleware { user }
    }
}

#[async_trait]
impl MiddleWareHandler for AuthorisationMiddleware {
    async fn pre_request(&self, req: &mut Request, _res: &mut Response) -> Result<()> {
        let mut extensions = req.extensions_mut();
        let cookies = req.cookies_mut();
        let pool = extensions.get::<Arc<MySqlPool>>()
            .ok_or(SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get database pool from request".to_string(),
            ))?;
        let store = extensions.get::<Arc<RwLock<MemoryStore>>>()
            .ok_or(SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get session from request".to_string(),
            ))?.read().await;
        let cookie = cookies.get("noice-web-session");
        if cookie.is_none() {
            extensions.insert(self.user.clone());
            return Ok(());
        }
        let cookie = cookie.unwrap();
        let session = store.load_session(
            cookie.value().to_string()
        ).await.map_err(
            |e| SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load session: {}", e),
            )
        )?;
        if let Some(session) = session {
            if let Some(user_id) = session.get::<i64>("user_id") {
                let user = User::fetch_by_id(pool, user_id).await?;
                extensions.insert(UserAuth::User(user));
                return Ok(());
            }
        }
        extensions.insert(self.user.clone());
        Ok(())
    }
}