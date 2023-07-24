mod login;
mod register;
mod update_password;
mod info;

use silent::prelude::{HandlerAppend, Route};

pub fn user_route() -> Route {
    Route::new("")
        .append(Route::new("register").post(register::register))
        .append(Route::new("login").post(login::login))
        .append(Route::new("updatePassword").post(update_password::update_password))
        .append(Route::new("info").get(info::info))
}
