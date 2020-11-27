use fake::Fake;
use radmin::uuid::Uuid;
use diesel::RunQueryDsl;
use fake::faker::name::en::Name;
use radmin::diesel::PgConnection;
use radmin::serde::{Deserialize, Serialize};

use crate::models::AddressBook;
use crate::schema::addressbooks;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "addressbooks"]
pub struct AddressBookFactory {
    pub account_id: Uuid,
    pub name: String,
}

impl AddressBookFactory {


    pub fn new(account_id: Uuid) -> AddressBookFactory {
        AddressBookFactory {
            account_id,
            name: Name().fake()
        }
    }

    pub fn insert(self, conn: &PgConnection) -> AddressBook {
        radmin::diesel::insert_into(addressbooks::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Category Factory")
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> AddressBookFactory {
        self.name = name.into();
        self
    }
}