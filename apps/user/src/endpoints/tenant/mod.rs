mod tenant_;
mod tenant_user;

use silent::prelude::{HandlerAppend, Route};

pub fn tenant_route() -> Route {
    Route::new("tenant")
        .append(Route::new("create").post(tenant_::create))
        .append(Route::new("update/<id:i64>").put(tenant_::update))
        .append(Route::new("delete").post(tenant_::delete))
        .append(Route::new("list").get(tenant_::list))
        .append(Route::new("info/<id:i64>").get(tenant_::info))
        .append(Route::new("user/<id:i64>").get(tenant_::user))
        .append(Route::new("user/create").post(tenant_user::create))
        .append(Route::new("user/update/<id:i64>").post(tenant_user::update))
        .append(Route::new("user/delete").post(tenant_user::delete))
        .append(Route::new("user/list").get(tenant_user::list))
        .append(Route::new("user/info/<id:i64>").get(tenant_user::info))
}