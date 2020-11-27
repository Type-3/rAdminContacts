use diesel::{PgConnection, RunQueryDsl};

use crate::models::contacts::ContactEmail;
use crate::schema::contact_emails;
use radmin::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "contact_emails"]
pub struct ContactEmailFactory {
    contact_id: Uuid,
    email_id: Uuid,
    email_type: String
}

impl ContactEmailFactory {
    pub fn new<S: Into<String>>(contact_id: Uuid, email_id: Uuid, email_type: S) -> ContactEmailFactory {
        ContactEmailFactory { contact_id, email_id, email_type: email_type.into() }
    }

    pub fn contact_id(mut self, s: Uuid) -> ContactEmailFactory {
        self.contact_id = s;
        self
    }

    pub fn email_id(mut self, s: Uuid) -> ContactEmailFactory {
        self.email_id = s;
        self
    }

    pub fn email_type<S: Into<String>>(mut self, s: S) -> ContactEmailFactory {
        self.email_type = s.into();
        self
    }

    pub fn insert(self, conn: &PgConnection) -> ContactEmail {
        radmin::diesel::insert_into(contact_emails::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}