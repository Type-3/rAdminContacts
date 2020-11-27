use radmin::cli_table::Cell;
use radmin::uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::schema::addressbooks;

#[derive(Debug, PartialEq, Clone, Identifiable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "addressbooks"]
pub struct AddressBook {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
}

impl Into<Vec<Cell>> for AddressBook {
    fn into(self) -> Vec<Cell> {
        vec![
            Cell::new(&self.id, Default::default()),
            Cell::new(&self.account_id, Default::default()),
            Cell::new(&self.name, Default::default()),
        ]
    }
}
