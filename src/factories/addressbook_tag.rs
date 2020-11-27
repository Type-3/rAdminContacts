use diesel::RunQueryDsl;
use fake::faker::name::en::Name;
use fake::Fake;
use radmin::diesel::PgConnection;
use radmin::serde::{Deserialize, Serialize};
use radmin::uuid::Uuid;

use crate::models::AddressBookTag;
use crate::schema::addressbook_tags;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "addressbook_tags"]
pub struct AddressBookTagFactory {
    pub addressbook_id: Uuid,
    pub label: String,
}

impl AddressBookTagFactory {
    pub fn new(addressbook_id: Uuid) -> AddressBookTagFactory {
        AddressBookTagFactory {
            addressbook_id,
            label: Name().fake(),
        }
    }

    pub fn insert(self, conn: &PgConnection) -> AddressBookTag {
        radmin::diesel::insert_into(addressbook_tags::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Category Factory")
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> AddressBookTagFactory {
        self.label = label.into();
        self
    }
}
