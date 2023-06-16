use async_trait::async_trait;
use silent::prelude::{MiddleWareHandler, Result, warn};
use silent::{Request, Response};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
use std::sync::Arc;

pub struct DatabaseMiddleware {
    pub db: Arc<MySqlPool>,
}

impl Default for DatabaseMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseMiddleware {
    pub fn new() -> Self {
        let pool =
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(
                    MySqlPoolOptions::new()
                        .max_connections(16)
                        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
                )
                .expect("Failed to connect to MySQL");
        let db = Arc::new(pool);
        DatabaseMiddleware { db }
    }
}

#[async_trait]
impl MiddleWareHandler for DatabaseMiddleware {
    async fn pre_request(&self, req: &mut Request, _res: &mut Response) -> Result<()> {
        warn!("db middleware running");
        req.extensions_mut().insert(self.db.clone());
        Ok(())
    }
}
