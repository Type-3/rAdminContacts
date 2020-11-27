use radmin::uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::schema::phone_numbers;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "phone_numbers"]
pub struct Phone {
    pub id: Uuid,
    pub phone: String,
    pub extension: String
}