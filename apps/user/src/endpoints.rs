use std::sync::Arc;
use serde::{Deserialize, Serialize};
use silent::{Request, Result, SilentError, StatusCode};
use silent::prelude::warn;
use sqlx::MySqlPool;
use noice_core::get_db;
use crate::models::User;

#[derive(Deserialize, Debug)]
struct RegisterRequest {
    username: String,
    password: String,
    name: Option<String>,
}

#[derive(Serialize, Debug)]
pub(crate) struct RegisterResponse {
    id: i64,
    username: String,
    name: Option<String>,
}

impl From<User> for RegisterResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
        }
    }
}

pub(crate) async fn register(mut req: Request) -> Result<RegisterResponse> {
    let register_request: RegisterRequest = req.json_parse().await?;
    let pool = get_db(&req)?;
    warn!("registering user: {:?}", register_request);
    warn!("pool: {:?}", pool);
    let user = User::registry(
        pool,
        register_request.username,
        register_request.password,
        register_request.name,
    ).await.map_err(
        |e| SilentError::business_error(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to register user: {}", e))
    )?.into();
    // let user = RegisterResponse {
    //     id: 0,
    //     username: "12345".to_string(),
    //     name: None,
    // };
    Ok(user)
}