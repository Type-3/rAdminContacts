use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::ContactInfo as Contact;
use crate::models::Email;
use crate::schema::contact_emails;

#[derive(
    Debug,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    AsChangeset,
    Associations,
)]
#[belongs_to(Contact)]
#[belongs_to(Email)]
#[table_name = "contact_emails"]
pub struct ContactEmail {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub email_id: Uuid,
    pub email_type: String,
}
