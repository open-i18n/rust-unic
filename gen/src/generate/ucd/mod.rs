mod shared;

pub mod age;
pub mod bidi;
pub mod category;
pub mod core;
pub mod normal;

pub use self::shared::unicode_data::{read_unicode_data, UnicodeData, UnicodeDataEntry};
pub use self::shared::version::{read_unicode_version, UnicodeVersion};
