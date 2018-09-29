// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![warn(
    bad_style,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    unconditional_recursion,
)]
#![forbid(unsafe_code)]

//! # UNIC — UCD — Name Aliases
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for character Name Aliases properties from Unicode Character Database (UCD)

extern crate unic_char_property;
extern crate unic_ucd_version;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

mod name_aliases;
pub use name_aliases::{
    name_corrections_of,
    control_code_names_of,
    alternate_names_of,
    figments_of,
    name_abbreviations_of,
};

use unic_ucd_version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
