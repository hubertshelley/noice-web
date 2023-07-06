use crate::endpoints;
use silent::prelude::{Route};

pub fn user_route() -> Route {
    Route::new("user")
        .append(endpoints::user::user_route())
}
