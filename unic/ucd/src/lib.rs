// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code, missing_docs, unconditional_recursion)]

//! # UNIC — Unicode Character Database
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! This UNIC component provides access to character properties as defined in the [Unicode
//! Standard Annex #44 - Unicode Character Database](http://unicode.org/reports/tr44/).


pub extern crate unic_ucd_core as core;
pub extern crate unic_ucd_age as age;
pub extern crate unic_ucd_bidi as bidi;
pub extern crate unic_ucd_normal as normal;
pub extern crate unic_ucd_category as category;


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub use core::UNICODE_VERSION;


/// UNIC component version.
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// UNIC component name.
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

/// UNIC component description.
pub const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");


pub use age::{Age, CharAge};
pub use bidi::{BidiClass, CharBidiClass, StrBidiClass};
pub use core::UnicodeVersion;
pub use normal::CanonicalCombiningClass;
