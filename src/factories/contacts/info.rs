use diesel::{PgConnection, RunQueryDsl};

use crate::models::contacts::ContactInfo;
use crate::schema::contacts;
use radmin::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "contacts"]
pub struct ContactInfoFactory {
    pub addressbook_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
}

impl ContactInfoFactory {
    pub fn new(addressbook_id: Uuid) -> ContactInfoFactory {
        ContactInfoFactory {
            addressbook_id,
            first_name: None,
            last_name: None,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
        }
    }

    pub fn first_name<S: Into<String>>(mut self, s: S) -> ContactInfoFactory {
        self.first_name = Some(s.into());
        self
    }

    pub fn last_name<S: Into<String>>(mut self, s: S) -> ContactInfoFactory {
        self.last_name = Some(s.into());
        self
    }

    pub fn middle_name<S: Into<String>>(mut self, s: S) -> ContactInfoFactory {
        self.middle_name = Some(s.into());
        self
    }

    pub fn name_prefix<S: Into<String>>(mut self, s: S) -> ContactInfoFactory {
        self.name_prefix = Some(s.into());
        self
    }

    pub fn name_suffix<S: Into<String>>(mut self, s: S) -> ContactInfoFactory {
        self.name_suffix = Some(s.into());
        self
    }

    pub fn insert(self, conn: &PgConnection) -> ContactInfo {
        radmin::diesel::insert_into(contacts::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}
