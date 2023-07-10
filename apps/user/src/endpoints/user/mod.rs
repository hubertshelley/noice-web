mod login;
mod register;
mod update_password;
mod info;

use std::sync::Arc;
use silent::Method;
use silent::prelude::{HandlerAppend, HandlerGetter, Route};

pub fn user_route() -> Route {
    Route::new("")
        .append(Route::new("register").post(register::register))
        .append(Route::new("login").handler(
            Method::POST,
            Arc::new(login::LoginEndpoint {}),
        ))
        .append(Route::new("updatePassword").post(update_password::update_password))
        .append(Route::new("info").get(info::info))
}
