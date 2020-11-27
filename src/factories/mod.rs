mod addressbook;
pub use self::addressbook::AddressBookFactory;

mod addressbook_tag;
pub use self::addressbook_tag::AddressBookTagFactory;

mod shared_addressbook;
pub use self::shared_addressbook::SharedAddressBookFactory;

mod email;
pub use self::email::EmailFactory;

mod phone;
pub use self::phone::PhoneFactory;

mod address;
pub use self::address::AddressFactory;

pub mod contacts;
pub mod organizations;
