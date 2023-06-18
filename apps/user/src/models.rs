use anyhow::Result;
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub(crate) struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
}

impl User {
    pub async fn registry(
        pool: &MySqlPool,
        username: String,
        password: String,
        name: Option<String>,
    ) -> Result<Self> {
        let user_id = sqlx::query!(
            r#"
            INSERT INTO noice_web_user (username, password, name)
            VALUES (?, ?, ?)
            "#,
            username.clone(),
            password.clone(),
            name.clone()
        )
        .execute(pool)
        .await?
        .last_insert_id();
        Ok(Self {
            id: user_id as i64,
            username,
            password,
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
        self.password == password
    }
    pub fn get_token(&self) -> String {
        format!("{}:{}", self.id, self.username)
    }
}
