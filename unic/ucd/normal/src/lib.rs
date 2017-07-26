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


#![deny(unsafe_code, missing_docs)]

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

extern crate unic_ucd_core;
#[cfg(feature = "unic-ucd-category")]
extern crate unic_ucd_category;


mod canonical_combining_class;
mod composition;
mod decompose;
mod gen_cat;
mod hangul;
mod decomposition_type;


pub use canonical_combining_class::CanonicalCombiningClass;
pub use composition::{canonical_decomposition, compatibility_decomposition, canonical_composition};
pub use gen_cat::is_combining_mark;
pub use decompose::{decompose_canonical, decompose_compatible};
pub use decomposition_type::DecompositionType;

use std::cmp::Ordering;

use unic_ucd_core::UnicodeVersion;


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("tables/unicode_version.rsv");

/// Compose two characters into a single character, if possible.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
pub fn compose(a: char, b: char) -> Option<char> {
    hangul::compose(a, b).or_else(|| match canonical_composition(a) {
        None => None,
        Some(candidates) => {
            match candidates.binary_search_by(|&(val, _)| if b == val {
                Ordering::Equal
            } else if val < b {
                Ordering::Less
            } else {
                Ordering::Greater
            }) {
                Ok(idx) => {
                    let (_, result) = candidates[idx];
                    Some(result)
                }
                Err(_) => None,
            }
        }
    })
}
