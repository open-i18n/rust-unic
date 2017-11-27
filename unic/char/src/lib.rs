// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! # UNIC — Utilities
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).

#![forbid(missing_docs)]
#![deny(unsafe_code)]


pub extern crate unic_char_property as property;


/// UNIC component version.
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// UNIC component name.
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// UNIC component description.
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
