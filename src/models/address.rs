use radmin::uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::schema::physical_addresses;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "physical_addresses"]
pub struct Address {
    pub id: Uuid,
    pub post_office_box: Option<String>,
    pub extension: Option<String>,
    pub street: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub code: Option<String>,
    pub country: Option<String>
}