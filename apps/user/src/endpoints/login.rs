use crate::models::User;
use noice_core::{get_db, set_cookie};
use serde::{Deserialize, Serialize};
use silent::{Handler, Request, Response, Result, SilentError, StatusCode};
use async_trait::async_trait;

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

struct LoginEndPoint;

#[async_trait]
impl Handler for LoginEndPoint {
    async fn call(&self, mut req: Request) -> Result<Response> {
        let login_request: LoginRequest = req.json_parse().await?;
        let pool = get_db(&req)?;
        let user = User::fetch_by_username(pool, login_request.username)
            .await
            .map_err(|e| {
                SilentError::business_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to Login user: {}", e),
                )
            })?;
        match user.check_password(login_request.password) {
            true => {
                let mut res = Response::empty();
                res.set_body(serde_json::to_vec(&user)?.into());
                set_cookie(&mut res, user.get_cookie());
                Ok(res)
            },
            false => Err(SilentError::business_error(
                StatusCode::UNAUTHORIZED,
                "Wrong password".to_string(),
            )),
        }
    }
}

pub(crate) async fn login(mut req: Request) -> Result<LoginResponse> {
    let login_request: LoginRequest = req.json_parse().await?;
    let pool = get_db(&req)?;
    let user = User::fetch_by_username(pool, login_request.username)
        .await
        .map_err(|e| {
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to Login user: {}", e),
            )
        })?;
    match user.check_password(login_request.password) {
        true => Ok(user.into()),
        false => Err(SilentError::business_error(
            StatusCode::UNAUTHORIZED,
            "Wrong password".to_string(),
        )),
    }
}
