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
#![warn(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    unconditional_recursion
)]
#![forbid(unsafe_code)]

//! # UNIC — UCD — Case Character Properties
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for case conversion (case folding) character properties from the Unicode Character
//! Database (UCD).

#[macro_use]
extern crate unic_char_property;

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd_version;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub mod lowercase;
pub use lowercase::{is_lowercase, Lowercase};

pub mod uppercase;
pub use uppercase::{is_uppercase, Uppercase};

pub mod cased;
pub use cased::{is_cased, Cased};

pub mod case_ignorable;
pub use case_ignorable::{is_case_ignorable, CaseIgnorable};

pub mod changes_when_lowercased;
pub use changes_when_lowercased::{changes_when_lowercased, ChangesWhenLowercased};

pub mod changes_when_uppercased;
pub use changes_when_uppercased::{changes_when_uppercased, ChangesWhenUppercased};

pub mod changes_when_titlecased;
pub use changes_when_titlecased::{changes_when_titlecased, ChangesWhenTitlecased};

pub mod changes_when_casefolded;
pub use changes_when_casefolded::{changes_when_casefolded, ChangesWhenCasefolded};

pub mod changes_when_casemapped;
pub use changes_when_casemapped::{changes_when_casemapped, ChangesWhenCasemapped};

use unic_ucd_version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
