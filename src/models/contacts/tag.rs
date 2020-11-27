use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::ContactInfo as Contact;
use crate::models::AddressBookTag as Tag;
use crate::schema::contact_tags;

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
#[belongs_to(Tag)]
#[table_name = "contact_tags"]
pub struct ContactTag {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub tag_id: Uuid,
}
