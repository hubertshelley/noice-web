mod db;
mod cookies;

pub use db::DatabaseMiddleware;
use silent::{Request, Result, SilentError, StatusCode};
use sqlx::MySqlPool;
use std::sync::Arc;

pub fn get_db(req: &Request) -> Result<&Arc<MySqlPool>> {
    req.extensions()
        .get::<Arc<MySqlPool>>()
        .ok_or(SilentError::business_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get database pool from request".to_string(),
        ))
}

pub use cookies::{set_cookie, get_cookie};