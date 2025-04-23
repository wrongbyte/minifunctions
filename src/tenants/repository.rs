use anyhow::Result;
use sqlx::PgPool;

use super::{routes::CreateTenantPayload, tenant::Tenant};

pub type DynTenantRepository = dyn TenantRepository + Send + Sync;

pub struct SqlTenantRepository {
    pub pool: PgPool,
}

impl TenantRepository for SqlTenantRepository {
    fn insert(&self, payload: CreateTenantPayload) -> Result<Tenant> {
        todo!()
    }
}

pub trait TenantRepository {
    fn insert(&self, payload: CreateTenantPayload) -> Result<Tenant>;
}
