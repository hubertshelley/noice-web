use silent::prelude::{HandlerAppend, Route};
use crate::endpoints;

pub fn user_route() -> Route {
    Route::new("").get(|_| async move { Ok("Hello, user!") }).append(
        Route::new("register").post(endpoints::register),
    )
}