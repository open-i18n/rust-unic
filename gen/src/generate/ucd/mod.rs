mod shared;

pub mod age;
pub mod bidi;
pub mod core;

pub use self::shared::unicode_data::{UNICODE_DATA, UnicodeDataEntry};
pub use self::shared::version::UNICODE_VERSION;
