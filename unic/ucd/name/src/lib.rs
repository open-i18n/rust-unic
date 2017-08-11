extern crate unic_ucd_core;
extern crate unic_utils;

use unic_ucd_core::UnicodeVersion;

pub const UNICODE_VERSION: UnicodeVersion = include!("tables/unicode_version.rsv");
mod data {
    include!("tables/name_values.rs");
    pub const NAMES: &[(char, &[&str])] = include!("tables/name_map.rsv");
}
pub use data::NAMES;
