use silent::{Request, Result, SilentError, StatusCode};
use noice_core::{get_db};
use noice_core::models::User;

pub async fn get_user(req: &Request) -> Result<User> {
    let pool = get_db(req)?;
    let cookies = req.cookies();
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