use serde::Deserialize;
use silent::{Request, Result};
use noice_core::get_db;
use noice_core::models::{CURD, Tenant, TenantUser};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTenant {
    pub name: String,
    pub owner_id: i64,
}

impl From<CreateTenant> for Tenant {
    fn from(item: CreateTenant) -> Self {
        Self {
            id: 0,
            name: item.name,
            owner_id: item.owner_id,
        }
    }
}

pub async fn create(mut req: Request) -> Result<Box<Tenant>> {
    let pool = get_db(&req)?.clone();
    let tenant = req.json_parse::<CreateTenant>().await?;
    let tenant = Tenant::create(&pool, tenant.into()).await?;
    Ok(tenant)
}

pub async fn update(mut req: Request) -> Result<Box<Tenant>> {
    let pool = get_db(&req)?.clone();
    let tenant = req.json_parse::<Tenant>().await?;
    let tenant = Tenant::update(&pool, tenant).await?;
    Ok(tenant)
}

pub async fn delete<'a>(req: Request) -> Result<&'a str> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i32>("id")?;
    Tenant::delete(&pool, id as i64).await?;
    Ok("ok")
}

pub async fn list(req: Request) -> Result<Vec<Box<Tenant>>> {
    let pool = get_db(&req)?.clone();
    let tenants = Tenant::fetch_all(&pool).await?;
    Ok(tenants)
}

pub async fn info(req: Request) -> Result<Box<Tenant>> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i32>("id")?;
    let tenant = Tenant::fetch_by_id(&pool, id as i64).await?;
    Ok(tenant)
}

pub async fn user(req: Request) -> Result<Vec<TenantUser>> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i32>("id")?;
    let tenant = Tenant::fetch_by_id(&pool, id as i64).await?;
    let users = tenant.fetch_users(&pool).await?;
    Ok(users)
}