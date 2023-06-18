use crate::endpoints;
use silent::prelude::{HandlerAppend, Route};

pub fn user_route() -> Route {
    Route::new("")
        .get(|_| async move { Ok("Hello, user!") })
        .append(Route::new("register").post(endpoints::register::register))
        .append(Route::new("login").post(endpoints::login::login))
}
