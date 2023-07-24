use noice_core::models::User;
use serde::Serialize;
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
    if let Some(UserAuth::User(user)) = req.extensions().get::<UserAuth>() {
        return Ok(RegisterResponse::from(user.clone()));
    }
    Err(SilentError::business_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to register user: {}", "e"),
    ))
}
