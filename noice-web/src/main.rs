use std::sync::Arc;
use silent::prelude::*;
use noice_web_app_user::user_route;

fn main() {
    logger::fmt().with_max_level(Level::DEBUG).init();
    let route = Route::new("")
        .append(
            Route::new("api").append(
                Route::new("user").append(
                    user_route()
                )
            )
        )
        .append(
            Route::new("<path:**>").handler(
                Method::GET,
                Arc::new(static_handler("static")),
            )
        );
    Server::new().bind_route(route).run();
}
