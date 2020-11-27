use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::schema::shared_addressbooks;

#[derive(Debug, PartialEq, Clone, Identifiable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "shared_addressbooks"]
pub struct SharedAddressBook {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub addressbook_id: Uuid,
    pub account_id: Uuid,
}
