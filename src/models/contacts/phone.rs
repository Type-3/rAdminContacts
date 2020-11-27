use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::ContactInfo as Contact;
use crate::models::Phone;
use crate::schema::contact_phones;

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
#[belongs_to(Contact)]
#[belongs_to(Phone)]
#[table_name = "contact_phones"]
pub struct ContactPhone {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub phone_id: Uuid,
    pub phone_type: String,
}
