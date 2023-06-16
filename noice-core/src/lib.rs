mod db;

use std::sync::Arc;
use silent::{Request, Result, SilentError, StatusCode};
use sqlx::MySqlPool;
pub use db::DatabaseMiddleware;

pub fn get_db(req: &Request) -> Result<&Arc<MySqlPool>> {
    req.extensions().get::<Arc<MySqlPool>>().ok_or(
        SilentError::business_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get database pool from request".to_string())
    )
}