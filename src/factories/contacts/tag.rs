use radmin::uuid::Uuid;
use diesel::RunQueryDsl;
use radmin::diesel::PgConnection;
use radmin::serde::{Deserialize, Serialize};

use crate::models::contacts::ContactTag;
use crate::schema::contact_tags;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "contact_tags"]
pub struct ContactTagFactory {
    pub contact_id: Uuid,
    pub tag_id: Uuid,
}

impl ContactTagFactory {


    pub fn new(contact_id: Uuid, tag_id: Uuid) -> ContactTagFactory {
        ContactTagFactory {
            contact_id,
            tag_id
        }
    }

    pub fn insert(self, conn: &PgConnection) -> ContactTag {
        radmin::diesel::insert_into(contact_tags::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Category Factory")
    }
}