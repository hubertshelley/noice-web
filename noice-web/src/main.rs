use dotenv::dotenv;
use noice_core::{AuthorisationMiddleware, DatabaseMiddleware, SessionMiddleware};
use noice_web_user::user_route;
use silent::prelude::*;
use std::sync::Arc;

fn main() {
    // 加载 .env 文件
    dotenv().ok();
    logger::fmt().with_max_level(Level::DEBUG).init();
    let database = DatabaseMiddleware::new();
    let session = SessionMiddleware::new();
    let auth = AuthorisationMiddleware::new();
    let route = Route::new("")
        .hook(database)
        .hook(session)
        .hook(auth)
        .append(Route::new("api").append(Route::new("user").append(user_route())))
        .append(Route::new("<path:**>").handler(Method::GET, Arc::new(static_handler("static"))));
    Server::new().bind_route(route).run();
}
