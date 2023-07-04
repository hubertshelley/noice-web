use crate::endpoints;
use silent::prelude::{HandlerAppend, Route};

pub fn user_route() -> Route {
    Route::new("")
        .append(Route::new("register").post(endpoints::register::register))
        .append(Route::new("login").post(endpoints::login::login))
        .append(Route::new("login").post(endpoints::login::login))
}
