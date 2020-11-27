mod addressbook;
pub use self::addressbook::AddressBook;

mod addressbook_tag;
pub use self::addressbook_tag::AddressBookTag;

mod shared_addressbook;
pub use self::shared_addressbook::SharedAddressBook;

mod email;
pub use self::email::Email;

mod phone;
pub use self::phone::Phone;

mod address;
pub use self::address::Address;

pub mod contacts;
pub mod organizations;
