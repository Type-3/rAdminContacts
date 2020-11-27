use diesel::{PgConnection, RunQueryDsl};

use crate::models::Address;
use crate::schema::physical_addresses;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "physical_addresses"]
pub struct AddressFactory {
    pub post_office_box: Option<String>,
    pub extension: Option<String>,
    pub street: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub code: Option<String>,
    pub country: Option<String>,
}

impl AddressFactory {
    pub fn post_office_box<S: Into<String>>(mut self, s: S) -> AddressFactory {
        self.post_office_box = Some(s.into());
        self
    }

    pub fn extension<S: Into<String>>(mut self, s: S) -> AddressFactory {
        self.extension = Some(s.into());
        self
    }

    pub fn street<S: Into<String>>(mut self, s: S) -> AddressFactory {
        self.street = Some(s.into());
        self
    }

    pub fn locality<S: Into<String>>(mut self, s: S) -> AddressFactory {
        self.locality = Some(s.into());
        self
    }

    pub fn region<S: Into<String>>(mut self, s: S) -> AddressFactory {
        self.region = Some(s.into());
        self
    }

    pub fn code<S: Into<String>>(mut self, s: S) -> AddressFactory {
        self.code = Some(s.into());
        self
    }

    pub fn country<S: Into<String>>(mut self, s: S) -> AddressFactory {
        self.country = Some(s.into());
        self
    }

    pub fn insert(self, conn: &PgConnection) -> Address {
        radmin::diesel::insert_into(physical_addresses::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}
