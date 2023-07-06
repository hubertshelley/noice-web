use silent::{Request, Result, SilentError, StatusCode};
use noice_core::{get_cookie, get_db};
use crate::models::User;

pub async fn get_user(req: &Request) -> Result<User> {
    let pool = get_db(req)?;
    let cookies = get_cookie(req)?;
    if let Some(id) = cookies.get("id").map(|c| c.value()) {
        User::fetch_by_id(pool, id.parse().map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse user id: {}", e),
            ))?).await
    } else {
        Err(
            SilentError::business_error(
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
            )
        )
    }
}