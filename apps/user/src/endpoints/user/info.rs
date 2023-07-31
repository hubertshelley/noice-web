use noice_core::models::User;
use serde::Serialize;
use silent::{Request, Result};
use noice_core::models::UserAuth;


#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    id: i64,
    username: String,
    name: Option<String>,
}

impl From<User> for RegisterResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
        }
    }
}

pub async fn info(req: Request) -> Result<RegisterResponse> {
    let user = req.extensions().get::<UserAuth>().unwrap().get_user()?;
    Ok(user.clone().into())
}
