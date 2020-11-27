use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::OrganizationInfo as Organization;
use crate::models::Email;
use crate::schema::organization_emails;

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
#[belongs_to(Organization)]
#[belongs_to(Email)]
#[table_name = "organization_emails"]
pub struct OrganizationEmail {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email_id: Uuid,
    pub email_type: String,
}
