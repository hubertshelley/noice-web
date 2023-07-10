use crate::models::User;
use noice_core::{get_db, models::User};
use serde::{Deserialize, Serialize};
use silent::{Request, Result, SilentError, StatusCode};

#[derive(Deserialize, Debug)]
struct RegisterRequest {
    username: String,
    password: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
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

pub async fn register(mut req: Request) -> Result<RegisterResponse> {
    let register_request: RegisterRequest = req.json_parse().await?;
    let pool = get_db(&req)?;
    let user = User::registry(
        pool,
        register_request.username,
        register_request.password,
        register_request.name,
    )
        .await
        .map_err(|e| {
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to register user: {}", e),
            )
        })?
        .into();
    Ok(user)
}
