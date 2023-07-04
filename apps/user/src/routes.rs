use std::sync::Arc;
use silent::Method;
use crate::endpoints;
use silent::prelude::{HandlerAppend, HandlerGetter, Route};

pub fn user_route() -> Route {
    Route::new("")
        .append(Route::new("register").post(endpoints::register::register))
        .append(Route::new("login").handler(
            Method::POST,
            Arc::new(endpoints::login::LoginEndpoint {}),
        ))
        .append(Route::new("updatePassword").post(endpoints::update_password::update_password))
}
