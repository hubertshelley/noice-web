mod tenant_;
mod tenant_user;

use silent::prelude::{HandlerAppend, Route};

pub fn tenant_route() -> Route {
    Route::new("tenant")
        .append(Route::new("create").post(tenant_::create))
        .append(Route::new("update").post(tenant_::update))
        .append(Route::new("delete").post(tenant_::delete))
        .append(Route::new("list").get(tenant_::list))
        .append(Route::new("info/<id:int>").get(tenant_::info))
        .append(Route::new("user/<id:int>").get(tenant_::user))
        .append(Route::new("user/create").post(tenant_user::create))
        .append(Route::new("user/update").post(tenant_user::update))
        .append(Route::new("user/delete").post(tenant_user::delete))
        .append(Route::new("user/list").get(tenant_user::list))
        .append(Route::new("user/info/<id:int>").get(tenant_user::info))
}