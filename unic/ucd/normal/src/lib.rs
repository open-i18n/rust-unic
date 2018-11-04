// Copyright 2012-2015 The Rust Project Developers.
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

//! # UNIC — UCD — Normalization
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Unicode character properties for composition and decomposition.
//!
//! ```rust
//! extern crate unic_ucd_normal;
//!
//! use unic_ucd_normal::compose;
//!
//! fn main() {
//!     assert_eq!(compose('A','\u{30a}'), Some('Å'));
//! }
//! ```

#[macro_use]
extern crate unic_char_property;
#[macro_use]
extern crate unic_char_range;
extern crate unic_ucd_hangul;
extern crate unic_ucd_version;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub mod canonical_combining_class;
pub use canonical_combining_class::CanonicalCombiningClass;

mod composition;
pub use composition::{
    canonical_composition,
    canonical_decomposition,
    compatibility_decomposition,
};

mod decomposition;
pub use decomposition::{decompose_canonical, decompose_compatible};

mod gen_cat;
pub use gen_cat::is_combining_mark;

mod decomposition_type;
pub use decomposition_type::DecompositionType;

use unic_ucd_hangul::compose_syllable;

/// Compose two characters into a single character, if possible.
/// See [Unicode Standard Annex #15](https://www.unicode.org/reports/tr15/)
/// for more information.
pub fn compose(a: char, b: char) -> Option<char> {
    compose_syllable(a, b).or_else(|| canonical_composition(a).and_then(|table| table.find(b)))
}

use unic_ucd_version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
