use diesel::{PgConnection, RunQueryDsl};

use crate::models::organizations::OrganizationInfo;
use crate::schema::organizations;

#[derive(Debug, PartialEq, Clone, Default, Insertable)]
#[table_name = "organizations"]
pub struct OrganizationInfoFactory {
    pub name: String,
}

impl OrganizationInfoFactory {
    pub fn new<S: Into<String>>(name: S) -> OrganizationInfoFactory {
        OrganizationInfoFactory { name: name.into() }
    }

    pub fn insert(self, conn: &PgConnection) -> OrganizationInfo {
        radmin::diesel::insert_into(organizations::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Physical Address")
    }
}
