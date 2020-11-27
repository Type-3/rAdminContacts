use radmin::uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::OrganizationInfo as Organization;
use crate::models::Address;
use crate::schema::organization_addresses;

#[derive(
    Debug, PartialEq, Clone, Serialize,
    Deserialize, Queryable, Identifiable,
    AsChangeset, Associations
)]
#[belongs_to(Organization)]
#[belongs_to(Address)]
#[table_name = "organization_addresses"]
pub struct OrganizationAddress {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub address_id: Uuid,
    pub address_type: String
}