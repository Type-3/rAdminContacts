use radmin::uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::schema::addressbook_tags;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "addressbook_tags"]
pub struct AddressBookTag {
    pub id: Uuid,
    pub addressbook_id: Uuid,
    pub label: String
}