use crate::models::User;
use noice_core::get_db;
use serde::{Deserialize};
use silent::{Request, Result};

#[derive(Deserialize, Debug)]
struct UpdatePasswordRequest {
    password: String,
}

pub(crate) async fn update_password(mut req: Request) -> Result<String> {
    let update_password_request: UpdatePasswordRequest = req.json_parse().await?;
    let pool = get_db(&req)?;
    let user = User::get_user(&req).await?;
    user.update_password(
        pool,
        update_password_request.password,
    ).await?;
    Ok("OK".to_string())
}
