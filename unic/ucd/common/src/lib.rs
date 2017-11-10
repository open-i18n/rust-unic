// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![no_std]
#![forbid(unsafe_code, unconditional_recursion, missing_docs)]

//! # UNIC — UCD — Case Character Properties
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for common character properties from the Unicode Character Database (UCD).


#[macro_use]
extern crate unic_char_property;

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd_version;


pub mod alphabetic;
pub mod alphanumeric;
pub mod control;
pub mod numeric;
pub mod white_space;


// UCD-defined: types and methods
pub use alphabetic::{is_alphabetic, Alphabetic};
pub use white_space::{is_white_space, WhiteSpace};

// Non-UCD-defined: methods only
pub use alphanumeric::is_alphanumeric;
pub use control::is_control;
pub use numeric::is_numeric;


use unic_ucd_version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
