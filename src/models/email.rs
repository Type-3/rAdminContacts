use radmin::uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::schema::email_addresses;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "email_addresses"]
pub struct Email {
    pub id: Uuid,
    pub account: String,
    pub domain: String
}