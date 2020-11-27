use diesel::RunQueryDsl;
use radmin::diesel::PgConnection;
use radmin::serde::{Deserialize, Serialize};

use crate::models::Email;
use crate::schema::email_addresses;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "email_addresses"]
pub struct EmailFactory {
    pub account: String,
    pub domain: String,
}

impl EmailFactory {


    pub fn new<S: Into<String>>(account: S, domain: S) -> EmailFactory {
        EmailFactory {
            account: account.into(),
            domain: domain.into()
        }
    }

    pub fn insert(self, conn: &PgConnection) -> Email {
        radmin::diesel::insert_into(email_addresses::table)
            .values(&self)
            .get_result(conn)
            .expect("Failed to insert new Category Factory")
    }

    pub fn account<S: Into<String>>(mut self, account: S) -> EmailFactory {
        self.account = account.into();
        self
    }

    pub fn domain<S: Into<String>>(mut self, domain: S) -> EmailFactory {
        self.domain = domain.into();
        self
    }
}