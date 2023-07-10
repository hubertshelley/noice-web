use std::sync::Arc;
use async_session::MemoryStore;
use silent::{MiddleWareHandler, Request, Response, Result};
use async_trait::async_trait;
use tokio::sync::RwLock;

pub struct SessionMiddleware {
    pub session: Arc<RwLock<MemoryStore>>,
}

impl Default for SessionMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionMiddleware {
    pub fn new() -> Self {
        let session = Arc::new(RwLock::new(MemoryStore::new()));
        SessionMiddleware { session }
    }
}

#[async_trait]
impl MiddleWareHandler for SessionMiddleware {
    async fn pre_request(&self, req: &mut Request, _res: &mut Response) -> Result<()> {
        req.extensions_mut().insert(self.session.clone());
        Ok(())
    }
}