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

//! # UNIC — UCD — Case Character Properties
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for common character properties from the Unicode Character Database (UCD).


#[macro_use]
extern crate unic_char_property;

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd_core;


pub mod alphabetic;
pub mod alphanumeric;
pub mod numeric;


pub use alphabetic::{is_alphabetic, Alphabetic};
pub use alphanumeric::is_alphanumeric;
pub use numeric::is_numeric;


use unic_ucd_core::UnicodeVersion;

/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
