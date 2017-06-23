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


#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # UNIC — UCD — Character Age
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for character [**Age**](http://www.unicode.org/reports/tr44/#Age) property from
//! Unicode Character Database (UCD)
//!
//! * <http://www.unicode.org/reports/tr44/#Character_Age>

extern crate unic_ucd_core;


mod tables;
mod traits;

pub use tables::UNICODE_VERSION;
pub use tables::Age;
pub use traits::CharAge;
