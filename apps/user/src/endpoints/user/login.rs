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

pub struct LoginEndpoint;

#[async_trait]
impl Handler for LoginEndpoint {
    async fn call(&self, mut req: Request) -> Result<Response> {
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
                let mut res = Response::empty();
                set_cookie(&mut res, user.get_cookie());
                let user: LoginResponse = user.into();
                res.set_body(serde_json::to_vec(&user)?.into());
                Ok(res)
            }
            false => Err(SilentError::business_error(
                StatusCode::UNAUTHORIZED,
                "Wrong password".to_string(),
            )),
        }
    }
}

#[allow(dead_code)]
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
