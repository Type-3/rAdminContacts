use radmin::uuid::Uuid;
use diesel::RunQueryDsl;
use radmin::diesel::PgConnection;
use radmin::serde::{Deserialize, Serialize};

use crate::models::SharedAddressBook;
use crate::schema::shared_addressbooks;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "shared_addressbooks"]
pub struct SharedAddressBookFactory {
    pub owner_id: Uuid,
    pub addressbook_id: Uuid,
    pub account_id: Uuid,
}

impl SharedAddressBookFactory {


    pub fn new(owner_id: Uuid, addressbook_id: Uuid, account_id: Uuid) -> SharedAddressBookFactory {
        SharedAddressBookFactory { owner_id, addressbook_id, account_id }
    }

    pub fn insert(self, conn: &PgConnection) -> SharedAddressBook {
        radmin::diesel::insert_into(shared_addressbooks::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Category Factory")
    }

    pub fn owner_id(mut self, owner_id: Uuid) -> SharedAddressBookFactory {
        self.owner_id = owner_id;
        self
    }

    pub fn addressbook_id(mut self, addressbook_id: Uuid) -> SharedAddressBookFactory {
        self.addressbook_id = addressbook_id;
        self
    }

    pub fn account_id(mut self, account_id: Uuid) -> SharedAddressBookFactory {
        self.account_id = account_id;
        self
    }
}