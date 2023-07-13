use async_trait::async_trait;
use serde::Serialize;
use sqlx::MySqlPool;
use silent::{Result, SilentError, StatusCode};
use crate::models::CURD;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Tenant {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct TenantUser {
    pub id: i64,
    pub user_id: i64,
    pub tenant_id: i64,
    pub employee_code: Option<String>,
    pub employee_name: Option<String>,
    pub nick_name: Option<String>,
}


impl Tenant {}


#[async_trait]
impl CURD for Tenant {
    async fn fetch_by_id(pool: &MySqlPool, id: i64) -> Result<Box<Self>> {
        let item = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM noice_web_tenant WHERE id = ?
            "#,
            id
        )
            .fetch_one(pool)
            .await.map_err(|e| SilentError::business_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch tenant: {}", e),
        ))?;
        Ok(Box::new(item))
    }

    async fn fetch_all(pool: &MySqlPool) -> Result<Vec<Box<Self>>> {
        let item = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM noice_web_tenant
            "#,
        )
            .fetch_all(pool)
            .await.map_err(|e| SilentError::business_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch tenant: {}", e),
        ))?;
        Ok(item.into_iter().map(Box::new).collect())
    }

    async fn create(pool: &MySqlPool, item: Self) -> Result<Box<Self>> {
        let insert_id = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO noice_web_tenant (name, owner_id)
            VALUES (?, ?)
            "#,
            item.name.clone(),
            item.owner_id.clone(),
        )
            .execute(pool)
            .await.map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create tenant: {}", e),
            ))?
            .last_insert_id();
        let item = Self {
            id: insert_id as i64,
            ..item
        };
        Ok(Box::new(item))
    }

    async fn update(pool: &MySqlPool, item: Self) -> Result<Box<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            UPDATE noice_web_tenant SET name = ?, owner_id = ? WHERE id = ?
            "#,
            item.name.clone(),
            item.owner_id.clone(),
            item.id.clone(),
        )
            .execute(pool)
            .await.map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create tenant: {}", e),
            ))?;
        Ok(Box::new(item))
    }

    async fn delete(pool: &MySqlPool, id: i64) -> Result<()> {
        sqlx::query_as!(
            Self,
            r#"
            DELETE FROM noice_web_tenant WHERE id = ?
            "#,
            id,
        )
            .execute(pool)
            .await.map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create tenant: {}", e),
            ))?;
        Ok(())
    }
}