mod tenant;
mod third_part;
mod user;

use async_trait::async_trait;
use silent::Result;
use sqlx::MySqlPool;
pub use user::{User, UserAuth};

#[async_trait]
pub trait CURD {
    async fn fetch_by_id(pool: &MySqlPool, id: i64) -> Result<Box<Self>>;
    async fn fetch_all(pool: &MySqlPool) -> Result<Vec<Box<Self>>>;
    async fn create(pool: &MySqlPool, item: Self) -> Result<Box<Self>>;
    async fn update(pool: &MySqlPool, item: Self) -> Result<Box<Self>>;
    async fn delete(pool: &MySqlPool, id: i64) -> Result<()>;
}