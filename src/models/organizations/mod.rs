mod info;
pub use self::info::OrganizationInfo;

mod tag;
pub use self::tag::OrganizationTag;

mod phone;
pub use self::phone::OrganizationPhone;

mod email;
pub use self::email::OrganizationEmail;

mod address;
pub use self::address::OrganizationAddress;

use diesel::{BelongingToDsl, PgConnection, QueryDsl, RunQueryDsl};
use radmin::uuid::Uuid;
use radmin::ServerError;

use crate::schema::organizations;

pub struct OrganizationRecord {
    pub organization: OrganizationInfo,
    pub addresses: Vec<OrganizationAddress>,
    pub emails: Vec<OrganizationEmail>,
    pub phones: Vec<OrganizationPhone>,
}

impl OrganizationRecord {
    pub fn from_id(id: Uuid, conn: &PgConnection) -> Result<OrganizationRecord, ServerError> {
        let organization: OrganizationInfo = organizations::table.find(id).first(conn)?;
        Ok(OrganizationRecord {
            emails: OrganizationEmail::belonging_to(&organization).load(conn)?,
            phones: OrganizationPhone::belonging_to(&organization).load(conn)?,
            addresses: OrganizationAddress::belonging_to(&organization).load(conn)?,
            organization,
        })
    }
}
