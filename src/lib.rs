#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

pub mod db;
pub mod factories;
pub mod models;
pub mod schema;

use radmin::crate_version;
use radmin::modules::{DatabaseModule, ServerModule};

#[derive(Default, Debug)]
pub struct ContactsModule;

impl ServerModule for ContactsModule {
    fn identifier(&self) -> &'static str {
        "rAdmin Contacts"
    }

    fn version(&self) -> &'static str {
        crate_version!()
    }

    fn database(&self) -> Box<dyn DatabaseModule> {
        Box::new(db::DbModule)
    }
}
