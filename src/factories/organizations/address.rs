use diesel::{PgConnection, RunQueryDsl};

use crate::models::organizations::OrganizationAddress;
use crate::schema::organization_addresses;
use radmin::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "organization_addresses"]
pub struct OrganizationAddressFactory {
    organization_id: Uuid,
    address_id: Uuid,
    address_type: String,
}

impl OrganizationAddressFactory {
    pub fn new<S: Into<String>>(organization_id: Uuid, address_id: Uuid, address_type: S) -> OrganizationAddressFactory {
        OrganizationAddressFactory { organization_id, address_id, address_type: address_type.into() }
    }

    pub fn organization_id(mut self, s: Uuid) -> OrganizationAddressFactory {
        self.organization_id = s;
        self
    }

    pub fn address_id(mut self, s: Uuid) -> OrganizationAddressFactory {
        self.address_id = s;
        self
    }

    pub fn address_type<S: Into<String>>(mut self, s: S) -> OrganizationAddressFactory {
        self.address_type = s.into();
        self
    }

    pub fn insert(self, conn: &PgConnection) -> OrganizationAddress {
        radmin::diesel::insert_into(organization_addresses::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}