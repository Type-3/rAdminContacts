use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::schema::contacts;
use radmin::chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "contacts"]
pub struct ContactInfo {
    pub id: Uuid,
    pub addressbook_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
