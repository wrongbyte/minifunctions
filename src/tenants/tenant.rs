use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Tenant {
    #[serde(rename = "tenant_id")]
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
