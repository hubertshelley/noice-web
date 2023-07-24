use async_session::Session;
use noice_core::{get_db, models::User};
use serde::{Deserialize, Serialize};
use silent::{Request, Result, SilentError, StatusCode};

#[derive(Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct LoginResponse {
    id: i64,
    token: String,
}

impl From<User> for LoginResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            token: user.get_token(),
        }
    }
}

#[allow(dead_code)]
pub(crate) async fn login(mut req: Request) -> Result<LoginResponse> {
    let login_request: LoginRequest = req.json_parse().await?;
    let pool = get_db(&req)?;
    let user = User::fetch_by_username(pool, login_request.username)
        .await
        .map_err(|_e| {
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "User not found".to_string(),
            )
        })?;
    match user.check_password(login_request.password) {
        true => {
            let session = req.extensions_mut().get_mut::<Session>().unwrap();
            session.insert("user_id", user.id)?;
            Ok(LoginResponse::from(user))
        }
        false => Err(SilentError::business_error(
            StatusCode::UNAUTHORIZED,
            "Wrong password".to_string(),
        )),
    }
}
