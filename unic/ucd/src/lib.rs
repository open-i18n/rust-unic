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
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    unconditional_recursion,
)]
#![forbid(unsafe_code)]

//! # UNIC — Unicode Character Database
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! This UNIC component provides access to character properties as defined in the [Unicode
//! Standard Annex #44 - Unicode Character Database](http://unicode.org/reports/tr44/).

pub extern crate unic_ucd_age as age;
pub extern crate unic_ucd_bidi as bidi;
pub extern crate unic_ucd_block as block;
pub extern crate unic_ucd_case as case;
pub extern crate unic_ucd_category as category;
pub extern crate unic_ucd_common as common;
pub extern crate unic_ucd_ident as ident;
pub extern crate unic_ucd_name as name;
pub extern crate unic_ucd_normal as normal;
pub extern crate unic_ucd_segment as segment;
pub extern crate unic_ucd_version as version;

pub use version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub use version::UNICODE_VERSION;

pub use age::{Age, CharAge};

pub use block::{Block, BlockIter};

pub use bidi::{is_bidi_mirrored, BidiClass, CharBidiClass, StrBidiClass};

pub use case::{
    changes_when_casefolded,
    changes_when_casemapped,
    changes_when_lowercased,
    changes_when_titlecased,
    changes_when_uppercased,
    is_case_ignorable,
    is_cased,
    is_lowercase,
    is_uppercase,
    CaseIgnorable,
    Cased,
    ChangesWhenCasefolded,
    ChangesWhenCasemapped,
    ChangesWhenLowercased,
    ChangesWhenTitlecased,
    ChangesWhenUppercased,
    Lowercase,
    Uppercase,
};

pub use category::GeneralCategory;

pub use common::{is_alphabetic, is_white_space, Alphabetic, WhiteSpace};

pub use name::Name;

pub use normal::CanonicalCombiningClass;

pub use segment::{GraphemeClusterBreak, SentenceBreak, WordBreak};

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};
