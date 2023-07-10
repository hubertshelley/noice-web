mod db;
mod session;
mod authorisation;

pub mod models;

pub use db::DatabaseMiddleware;
pub use session::SessionMiddleware;
use silent::{Request, Result, SilentError, StatusCode};
use sqlx::MySqlPool;
use std::sync::Arc;
use async_session::SessionStore;
use tokio::sync::RwLock;

pub fn get_db(req: &Request) -> Result<&Arc<MySqlPool>> {
    req.extensions()
        .get::<Arc<MySqlPool>>()
        .ok_or(SilentError::business_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get database pool from request".to_string(),
        ))
}

pub fn get_session<T>(req: &Request) -> Result<&Arc<RwLock<T>>>
    where
        T: SessionStore {
    req.extensions()
        .get::<Arc<RwLock<T>>>()
        .ok_or(SilentError::business_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get session from request".to_string(),
        ))
}
