// Copyright 2016 The rust-url developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    unconditional_recursion
)]
#![forbid(unsafe_code)]

//! # UNIC — IDNA — IDNA Mapping Table
//!
//! Mapping each Unicode codepoint to an IDNA status.
//!
//! * <https://www.unicode.org/reports/tr46/#IDNA_Mapping_Table>

extern crate unic_char_property;
#[macro_use]
extern crate unic_char_range;
extern crate unic_ucd_version;

mod mapping;

use unic_ucd_version::UnicodeVersion;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub use mapping::Mapping;

/// The version of [Unicode IDNA Compatibility Processing](https://www.unicode.org/reports/tr46/)
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
