use diesel::{PgConnection, RunQueryDsl};

use crate::models::organizations::OrganizationPhone;
use crate::schema::organization_phones;
use radmin::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "organization_phones"]
pub struct OrganizationPhoneFactory {
    organization_id: Uuid,
    phone_id: Uuid,
    phone_type: String,
}

impl OrganizationPhoneFactory {
    pub fn new<S: Into<String>>(organization_id: Uuid, phone_id: Uuid, phone_type: S) -> OrganizationPhoneFactory {
        OrganizationPhoneFactory { organization_id, phone_id, phone_type: phone_type.into() }
    }

    pub fn organization_id(mut self, s: Uuid) -> OrganizationPhoneFactory {
        self.organization_id = s;
        self
    }

    pub fn phone_id(mut self, s: Uuid) -> OrganizationPhoneFactory {
        self.phone_id = s;
        self
    }

    pub fn phone_type<S: Into<String>>(mut self, s: S) -> OrganizationPhoneFactory {
        self.phone_type = s.into();
        self
    }

    pub fn insert(self, conn: &PgConnection) -> OrganizationPhone {
        radmin::diesel::insert_into(organization_phones::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}