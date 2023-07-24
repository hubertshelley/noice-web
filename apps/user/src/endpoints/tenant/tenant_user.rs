use serde::Deserialize;
use silent::{Request, Result};
use noice_core::get_db;
use noice_core::models::{CURD, TenantUser};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTenantUser {
    pub user_id: i64,
    pub tenant_id: i64,
    pub employee_code: Option<String>,
    pub employee_name: Option<String>,
    pub nick_name: Option<String>,
}

impl From<CreateTenantUser> for TenantUser {
    fn from(item: CreateTenantUser) -> Self {
        Self {
            id: 0,
            user_id: item.user_id,
            tenant_id: item.tenant_id,
            employee_code: item.employee_code,
            employee_name: item.employee_name,
            nick_name: item.nick_name,
        }
    }
}

pub async fn create(mut req: Request) -> Result<Box<TenantUser>> {
    let pool = get_db(&req)?.clone();
    let tenant = req.json_parse::<CreateTenantUser>().await?;
    let tenant = TenantUser::create(&pool, tenant.into()).await?;
    Ok(tenant)
}

pub async fn update(mut req: Request) -> Result<Box<TenantUser>> {
    let pool = get_db(&req)?.clone();
    let tenant = req.json_parse::<TenantUser>().await?;
    let tenant = TenantUser::update(&pool, tenant).await?;
    Ok(tenant)
}

pub async fn delete<'a>(req: Request) -> Result<&'a str> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i32>("id")?;
    TenantUser::delete(&pool, id as i64).await?;
    Ok("ok")
}

pub async fn list(req: Request) -> Result<Vec<Box<TenantUser>>> {
    let pool = get_db(&req)?.clone();
    let tenants = TenantUser::fetch_all(&pool).await?;
    Ok(tenants)
}

pub async fn info(req: Request) -> Result<Box<TenantUser>> {
    let pool = get_db(&req)?.clone();
    let id = req.get_path_params::<i32>("id")?;
    let tenant = TenantUser::fetch_by_id(&pool, id as i64).await?;
    Ok(tenant)
}