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

//! # UNIC — UCD — Character Age
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for character [**Age**](http://www.unicode.org/reports/tr44/#Age) property from
//! Unicode Character Database (UCD)
//!
//! * <http://www.unicode.org/reports/tr44/#Character_Age>


/// UNIC component version.
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// UNIC component name.
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

/// UNIC component description.
pub const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");


mod domain;
mod range;
mod macros;


pub use self::domain::{CharProperty, CompleteCharProperty, PartialCharProperty};
pub use self::range::{EnumeratedCharProperty, NumericCharProperty, NumericCharPropertyValue};
