use tokio::sync::RwLockWriteGuard;
use async_session::{MemoryStore, SessionStore};
use crate::models::User;
use noice_core::{get_db, get_session, models::User};
use serde::{Deserialize, Serialize};
use silent::{Handler, Request, Response, Result, SilentError, StatusCode};
use async_trait::async_trait;
use cookie::Cookie;

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
                let store: RwLockWriteGuard<MemoryStore> = get_session(&req)?.write().await;
                let mut res = Response::empty();
                let session = user.get_session()?;
                let cookie_value = store.store_session(session).await.map_err(
                    |e| SilentError::business_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to store session: {}", e),
                    )
                )?;
                if let Some(cookie_value) = cookie_value {
                    res.cookies_mut().add(
                        Cookie::build("noice-web-session", cookie_value)
                            .max_age(cookie::time::Duration::hours(2))
                            .finish()
                    );
                    let user: LoginResponse = user.into();
                    res.set_body(serde_json::to_vec(&user)?.into());
                    Ok(res)
                } else {
                    Err(SilentError::business_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to store session".to_string(),
                    ))
                }
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
