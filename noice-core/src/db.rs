use async_trait::async_trait;
use silent::prelude::{MiddleWareHandler, Result};
use silent::{Request, Response};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
use std::sync::Arc;

pub struct DatabaseMiddleware {
    pub db: Arc<MySqlPool>,
}

impl DatabaseMiddleware {
    pub async fn new() -> Self {
        let pool = MySqlPoolOptions::new()
            .max_connections(16)
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .expect("Failed to connect to MySQL");
        let db = Arc::new(pool);
        DatabaseMiddleware { db }
    }
}

#[async_trait]
impl MiddleWareHandler for DatabaseMiddleware {
    async fn pre_request(&self, req: &mut Request, _res: &mut Response) -> Result<()> {
        req.extensions_mut().insert(self.db.clone());
        Ok(())
    }
}
