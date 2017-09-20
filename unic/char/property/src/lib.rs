// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code, unconditional_recursion, missing_docs)]

//! # UNIC - Unicode Characters - Character Property
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Character Property taxonomy, contracts and build macros.
//!
//! ## References
//!
//! * [Unicode UTR #23: The Unicode Character Property Model](http://unicode.org/reports/tr23/>).
//!
//! * [Unicode UAX #44: Unicode Character Database](http://unicode.org/reports/tr44/).
//!
//! * [UCD's PropertyAliases.txt](http://www.unicode.org/Public/UCD/latest/ucd/PropertyAliases.txt).


#[doc(hidden)]
#[allow(unused)] // used by char_property macro
pub extern crate unic_utils;


/// UNIC component version.
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// UNIC component name.
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

/// UNIC component description.
pub const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");


mod macros;
mod property;
mod range_types;


pub use self::property::{CharProperty, PartialCharProperty, TotalCharProperty};
pub use self::range_types::{
    BinaryCharProperty,
    CustomCharProperty,
    EnumeratedCharProperty,
    NumericCharProperty,
    NumericCharPropertyValue,
};
