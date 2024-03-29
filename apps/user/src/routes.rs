use crate::endpoints;
use silent::prelude::{Route};

pub fn user_route() -> Route {
    Route::new("")
        .append(endpoints::user::user_route())
        .append(endpoints::tenant::tenant_route())
}
