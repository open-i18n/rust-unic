mod shared;

pub mod age;
pub mod bidi;
pub mod core;

pub use self::shared::unicode_data::{read_unicode_data, UnicodeData, UnicodeDataEntry};
pub use self::shared::version::UNICODE_VERSION;
