use diesel::{PgConnection, RunQueryDsl};

use crate::models::contacts::ContactPhone;
use crate::schema::contact_phones;
use radmin::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "contact_phones"]
pub struct ContactPhoneFactory {
    contact_id: Uuid,
    phone_id: Uuid,
    phone_type: String,
}

impl ContactPhoneFactory {
    pub fn new<S: Into<String>>(contact_id: Uuid, phone_id: Uuid, phone_type: S) -> ContactPhoneFactory {
        ContactPhoneFactory { contact_id, phone_id, phone_type: phone_type.into() }
    }

    pub fn contact_id(mut self, s: Uuid) -> ContactPhoneFactory {
        self.contact_id = s;
        self
    }

    pub fn phone_id(mut self, s: Uuid) -> ContactPhoneFactory {
        self.phone_id = s;
        self
    }

    pub fn phone_type<S: Into<String>>(mut self, s: S) -> ContactPhoneFactory {
        self.phone_type = s.into();
        self
    }

    pub fn insert(self, conn: &PgConnection) -> ContactPhone {
        radmin::diesel::insert_into(contact_phones::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}