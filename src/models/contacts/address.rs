use radmin::uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::ContactInfo as Contact;
use crate::models::Address;
use crate::schema::contact_addresses;

#[derive(
    Debug, PartialEq, Clone, Serialize,
    Deserialize, Queryable, Identifiable,
    AsChangeset, Associations
)]
#[belongs_to(Contact)]
#[belongs_to(Address)]
#[table_name = "contact_addresses"]
pub struct ContactAddress {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub address_id: Uuid,
    pub address_type: String
}