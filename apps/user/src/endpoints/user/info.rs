use noice_core::{get_db, models::User};
use serde::{Deserialize, Serialize};
use silent::{Request, Result, SilentError, StatusCode};
use noice_core::models::UserAuth;


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

pub async fn info(req: Request) -> Result<RegisterResponse> {
    if let Some(user_auth) = req.extensions().get::<UserAuth>() {
        if let UserAuth::User(user) = user_auth {
            return Ok(RegisterResponse::from(user.clone()));
        }
    }
    Err(SilentError::business_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to register user: {}", "e"),
    ))
}
