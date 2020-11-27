use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::OrganizationInfo as Organization;
use crate::models::Phone;
use crate::schema::organization_phones;

#[derive(
    Debug,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    AsChangeset,
    Associations,
)]
#[belongs_to(Organization)]
#[belongs_to(Phone)]
#[table_name = "organization_phones"]
pub struct OrganizationPhone {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub phone_id: Uuid,
    pub phone_type: String,
}
