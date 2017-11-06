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
//! Accessor for case conversion (case folding) character properties from the Unicode Character
//! Database (UCD).


#[macro_use]
extern crate unic_char_property;

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd_version;


pub mod lowercase;
pub mod uppercase;
pub mod cased;
pub mod case_ignorable;
pub mod changes_when_lowercased;
pub mod changes_when_uppercased;
pub mod changes_when_titlecased;
pub mod changes_when_casefolded;
pub mod changes_when_casemapped;


pub use lowercase::{is_lowercase, Lowercase};
pub use uppercase::{is_uppercase, Uppercase};
pub use cased::{is_cased, Cased};
pub use case_ignorable::{is_case_ignorable, CaseIgnorable};

pub use changes_when_lowercased::{changes_when_lowercased, ChangesWhenLowercased};
pub use changes_when_uppercased::{changes_when_uppercased, ChangesWhenUppercased};
pub use changes_when_titlecased::{changes_when_titlecased, ChangesWhenTitlecased};
pub use changes_when_casefolded::{changes_when_casefolded, ChangesWhenCasefolded};
pub use changes_when_casemapped::{changes_when_casemapped, ChangesWhenCasemapped};


use unic_ucd_version::UnicodeVersion;

/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
