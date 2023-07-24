use std::sync::Arc;
use async_session::Session;
use silent::{MiddleWareHandler, Request, Response, Result, SilentError, StatusCode};
use async_trait::async_trait;
use sqlx::MySqlPool;
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
        let extensions = req.extensions_mut();
        let pool = extensions.get::<Arc<MySqlPool>>()
            .ok_or(SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get database pool from request".to_string(),
            ))?;
        if let Some(session) = extensions.get::<Session>() {
            if let Some(user_id) = session.get::<i64>("user_id") {
                let user = User::fetch_by_id(pool, user_id).await.map_err(
                    |e| {
                        SilentError::business_error(
                            StatusCode::UNAUTHORIZED,
                            e.to_string(),
                        )
                    }
                )?;
                extensions.insert(UserAuth::User(user));
                return Ok(());
            }
        }
        extensions.insert(self.user.clone());
        Ok(())
    }
}