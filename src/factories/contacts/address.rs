use diesel::{PgConnection, RunQueryDsl};

use crate::models::contacts::ContactAddress;
use crate::schema::contact_addresses;
use radmin::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "contact_addresses"]
pub struct ContactAddressFactory {
    contact_id: Uuid,
    address_id: Uuid,
    address_type: String,
}

impl ContactAddressFactory {
    pub fn new<S: Into<String>>(contact_id: Uuid, address_id: Uuid, address_type: S) -> ContactAddressFactory {
        ContactAddressFactory { contact_id, address_id, address_type: address_type.into() }
    }

    pub fn contact_id(mut self, s: Uuid) -> ContactAddressFactory {
        self.contact_id = s;
        self
    }

    pub fn address_id(mut self, s: Uuid) -> ContactAddressFactory {
        self.address_id = s;
        self
    }

    pub fn address_type<S: Into<String>>(mut self, s: S) -> ContactAddressFactory {
        self.address_type = s.into();
        self
    }

    pub fn insert(self, conn: &PgConnection) -> ContactAddress {
        radmin::diesel::insert_into(contact_addresses::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}