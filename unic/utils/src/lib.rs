// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Utilities for use in UNIC development.


#![forbid(missing_docs)]
#![deny(unsafe_code)]

//! # UNIC — Utilities
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).


/// UNIC component version.
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// UNIC component name.
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

/// UNIC component description.
pub const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");


pub mod char_property;
pub mod codepoints;
mod macros;
pub mod tables;


pub use codepoints::iter_all_chars;
pub use tables::CharDataTable;
