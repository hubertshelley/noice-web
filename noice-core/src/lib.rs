mod db;
mod authorisation;

pub mod models;

pub use db::DatabaseMiddleware;
pub use authorisation::AuthorisationMiddleware;
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
