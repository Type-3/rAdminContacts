mod info;
pub use self::info::ContactInfoFactory;

mod email;
pub use self::email::ContactEmailFactory;

mod phone;
pub use self::phone::ContactPhoneFactory;

mod address;
pub use self::address::ContactAddressFactory;

mod tag;
pub use self::tag::ContactTagFactory;