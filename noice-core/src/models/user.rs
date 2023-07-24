use async_session::Session;
use cookie::{Cookie};
use cookie::time::Duration;
use serde::Serialize;
use silent::prelude::argon2::{make_password, verify_password};
use silent::{SilentError, Result, StatusCode};
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub enum UserAuth {
    User(User),
    AnyOneUser,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: Option<String>,
    pub name: Option<String>,
}

impl User {
    pub async fn registry(
        pool: &MySqlPool,
        username: String,
        password: Option<String>,
        name: Option<String>,
    ) -> Result<Self> {
        let mut passwd = None;
        if let Some(password) = password {
            passwd = Some(make_password(password)?);
        }
        let user_id = sqlx::query!(
            r#"
            INSERT INTO noice_web_user (username, password, name)
            VALUES (?, ?, ?)
            "#,
            username.clone(),
            passwd,
            name.clone()
        )
            .execute(pool)
            .await.map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create user: {}", e),
            ))?
            .last_insert_id();
        Ok(Self {
            id: user_id as i64,
            username,
            password: passwd,
            name,
        })
    }
    pub async fn fetch_by_username(pool: &MySqlPool, username: String) -> Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM noice_web_user WHERE username = ?
            "#,
            username
        )
            .fetch_one(pool)
            .await.map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch user: {}", e),
            ))?;
        Ok(user)
    }
    pub async fn fetch_by_id(pool: &MySqlPool, id: i64) -> Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM noice_web_user WHERE id = ?
            "#,
            id
        )
            .fetch_one(pool)
            .await.map_err(|e| SilentError::business_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch user: {}", e),
        ))?;
        Ok(user)
    }
    pub fn check_password(&self, password: String) -> bool {
        if let Some(passwd) = self.password.clone() {
            verify_password(
                passwd,
                password,
            ).unwrap_or(false)
        } else {
            false
        }
    }
    pub async fn update_password(&self, pool: &MySqlPool, password: String) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE noice_web_user SET password = ? WHERE id = ?
            "#,
            make_password(password)?,
            self.id
        )
            .fetch_one(pool)
            .await.map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update password: {}", e),
            ))?;
        Ok(())
    }
    pub fn get_cookie(&self) -> Cookie {
        Cookie::build("id", self.id.to_string()).path("/").max_age(Duration::hours(2)).finish()
    }
    pub fn set_session(&self, mut session: Session) -> Result<Session> {
        session.insert("user_id", self.id)?;
        Ok(session)
    }
    pub fn get_token(&self) -> String {
        format!("{}:{}", self.id, self.username)
    }
}
