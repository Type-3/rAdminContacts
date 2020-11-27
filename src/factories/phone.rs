use diesel::RunQueryDsl;
use radmin::diesel::PgConnection;
use radmin::serde::{Deserialize, Serialize};

use crate::models::Phone;
use crate::schema::phone_numbers;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "phone_numbers"]
pub struct PhoneFactory {
    pub phone: String,
    pub extension: Option<String>,
}

impl PhoneFactory {
    pub fn new<S: Into<String>>(phone: S) -> PhoneFactory {
        PhoneFactory {
            phone: phone.into(),
            extension: None,
        }
    }

    pub fn insert(self, conn: &PgConnection) -> Phone {
        radmin::diesel::insert_into(phone_numbers::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Category Factory")
    }

    pub fn phone<S: Into<String>>(mut self, phone: S) -> PhoneFactory {
        self.phone = phone.into();
        self
    }

    pub fn domain<S: Into<String>>(mut self, extension: S) -> PhoneFactory {
        self.extension = Some(extension.into());
        self
    }
}
