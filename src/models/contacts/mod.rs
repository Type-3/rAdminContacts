mod info;
pub use self::info::ContactInfo;

mod phone;
pub use self::phone::ContactPhone;

mod address;
pub use self::address::ContactAddress;

mod email;
pub use self::email::ContactEmail;

mod tag;
pub use self::tag::ContactTag;

use radmin::ServerError;
use radmin::uuid::Uuid;
use diesel::{PgConnection, RunQueryDsl, QueryDsl, BelongingToDsl};

use crate::schema::contacts;

pub struct ContactRecord {
    pub contact: ContactInfo,
    pub tags: Vec<ContactTag>,
    pub addresses: Vec<ContactAddress>,
    pub emails: Vec<ContactEmail>,
    pub phones: Vec<ContactPhone>,
}

impl ContactRecord {

    pub fn from_id(id: Uuid, conn: &PgConnection) -> Result<ContactRecord, ServerError> {
        let contact: ContactInfo = contacts::table.find(id).first(conn)?;
        Ok(
            ContactRecord {
                emails: ContactEmail::belonging_to(&contact).load(conn)?,
                tags: ContactTag::belonging_to(&contact).load(conn)?,
                phones: ContactPhone::belonging_to(&contact).load(conn)?,
                addresses: ContactAddress::belonging_to(&contact).load(conn)?,
                contact
            }
        )
    }
}