use radmin::uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::schema::organizations;
use radmin::chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "organizations"]
pub struct OrganizationInfo {
    pub id: Uuid,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}