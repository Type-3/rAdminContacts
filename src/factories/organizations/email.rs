use diesel::{PgConnection, RunQueryDsl};

use crate::models::organizations::OrganizationEmail;
use crate::schema::organization_emails;
use radmin::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "organization_emails"]
pub struct OrganizationEmailFactory {
    organization_id: Uuid,
    email_id: Uuid,
    email_type: String
}

impl OrganizationEmailFactory {
    pub fn new<S: Into<String>>(organization_id: Uuid, email_id: Uuid, email_type: S) -> OrganizationEmailFactory {
        OrganizationEmailFactory { organization_id, email_id, email_type: email_type.into() }
    }

    pub fn organization_id(mut self, s: Uuid) -> OrganizationEmailFactory {
        self.organization_id = s;
        self
    }

    pub fn email_id(mut self, s: Uuid) -> OrganizationEmailFactory {
        self.email_id = s;
        self
    }

    pub fn email_type<S: Into<String>>(mut self, s: S) -> OrganizationEmailFactory {
        self.email_type = s.into();
        self
    }

    pub fn insert(self, conn: &PgConnection) -> OrganizationEmail {
        radmin::diesel::insert_into(organization_emails::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}