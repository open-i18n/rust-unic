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

//! # UNIC — Unicode Character Database
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! This UNIC component provides access to character properties as defined in the [Unicode
//! Standard Annex #44 - Unicode Character Database](http://unicode.org/reports/tr44/).

pub use unic_ucd_common as common;
pub use unic_ucd_version as version;

pub use unic_ucd_age as age;
pub use unic_ucd_bidi as bidi;
pub use unic_ucd_block as block;
pub use unic_ucd_case as case;
pub use unic_ucd_category as category;
pub use unic_ucd_hangul as hangul;
pub use unic_ucd_ident as ident;
pub use unic_ucd_name as name;
pub use unic_ucd_name_aliases as name_aliases;
pub use unic_ucd_normal as normal;
pub use unic_ucd_segment as segment;

pub use crate::version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub use crate::version::UNICODE_VERSION;

pub use crate::age::{Age, CharAge};

pub use crate::block::{Block, BlockIter};

pub use crate::bidi::{is_bidi_mirrored, BidiClass, CharBidiClass, StrBidiClass};

pub use crate::case::{
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

pub use crate::category::GeneralCategory;

pub use crate::common::{is_alphabetic, is_white_space, Alphabetic, WhiteSpace};

pub use crate::name::Name;

pub use crate::normal::CanonicalCombiningClass;

pub use crate::name_aliases::{name_aliases_of, NameAliasType};

pub use crate::segment::{GraphemeClusterBreak, SentenceBreak, WordBreak};

mod pkg_info;
pub use crate::pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};
