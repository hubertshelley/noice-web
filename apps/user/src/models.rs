use anyhow::Result;
use cookie::{Cookie, CookieJar};
use serde::Serialize;
use silent::prelude::argon2::{make_password, verify_password};
use sqlx::MySqlPool;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub(crate) struct User {
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
            passwd = Some(make_password(password.clone())?);
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
            .await?
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
            .await?;
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
            .await?;
        Ok(())
    }
    pub fn get_cookie(&self) -> CookieJar {
        let mut jar = CookieJar::new();
        jar.add(
            Cookie::new("id", self.id.to_string())
        );
        jar
    }
    pub fn get_token(&self) -> String {
        format!("{}:{}", self.id, self.username)
    }
}
