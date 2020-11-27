use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::schema::organization_tags;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "organization_tags"]
pub struct OrganizationTag {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub tag_id: Uuid,
}
