use serde::Deserialize;
use silent::{Request, Result, SilentError, StatusCode};
use noice_core::get_db;
use noice_core::models::{CURD, Tenant, TenantUser, UserAuth};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTenant {
    pub name: String,
    pub owner_id: Option<i64>,
}

pub async fn create(mut req: Request) -> Result<Box<Tenant>> {
    let pool = get_db(&req)?.clone();
    let tenant = req.json_parse::<CreateTenant>().await?;
    let user = req.extensions().get::<UserAuth>().unwrap().get_user()?;
    let tenant = Tenant::create(&pool, Tenant {
        id: 0,
        name: tenant.name,
        owner_id: tenant.owner_id.unwrap_or(
            user.id
        ),
    }).await?;
    Ok(tenant)
}

pub async fn update(mut req: Request) -> Result<Box<Tenant>> {
    let pool = get_db(&req)?.clone();
    let mut tenant = req.json_parse::<Tenant>().await?;
    let id = req.get_path_params::<i64>("id")?;
    let user = req.extensions().get::<UserAuth>().unwrap().get_user()?;
    if Tenant::fetch_by_id(&pool, id).await?.owner_id != user.id {
        return Err(
            SilentError::business_error(
                StatusCode::BAD_REQUEST,
                "不是您的组织，无法进行操作".to_string(),
            )
        );
    }
    tenant.id = id;
    let tenant = Tenant::update(&pool, tenant).await?;
    Ok(tenant)
}

pub async fn delete<'a>(req: Request) -> Result<&'a str> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i64>("id")?;
    let user = req.extensions().get::<UserAuth>().unwrap().get_user()?;
    if Tenant::fetch_by_id(&pool, id).await?.owner_id != user.id {
        return Err(
            SilentError::business_error(
                StatusCode::BAD_REQUEST,
                "不是您的组织，无法进行操作".to_string(),
            )
        );
    }
    Tenant::delete(&pool, id).await?;
    Ok("ok")
}

pub async fn list(req: Request) -> Result<Vec<Box<Tenant>>> {
    let pool = get_db(&req)?.clone();
    let tenants = Tenant::fetch_all(&pool).await?;
    Ok(tenants)
}

pub async fn info(req: Request) -> Result<Box<Tenant>> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i64>("id")?;
    let tenant = Tenant::fetch_by_id(&pool, id).await?;
    Ok(tenant)
}

pub async fn user(req: Request) -> Result<Vec<TenantUser>> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i64>("id")?;
    let tenant = Tenant::fetch_by_id(&pool, id).await?;
    let users = tenant.fetch_users(&pool).await?;
    Ok(users)
}