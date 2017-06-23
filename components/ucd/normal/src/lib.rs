// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![deny(unsafe_code)]
#![forbid(missing_docs)]

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


mod composition;
mod gen_cat;
mod hangul;

pub use composition::{canonical_decomposition, compatibility_decomposition, canonical_composition,
                      canonical_combining_class};
pub use gen_cat::is_combining_mark;

use std::cmp::Ordering;
use std::ops::FnMut;

use unic_ucd_core::UnicodeVersion;


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("tables/unicode_version.rsv");


/// Compute canonical Unicode decomposition for character.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
pub fn decompose_canonical<F>(c: char, mut i: F)
where
    F: FnMut(char),
{
    d(c, &mut i, false);
}

/// Compute canonical or compatible Unicode decomposition for character.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
pub fn decompose_compatible<F>(c: char, mut i: F)
where
    F: FnMut(char),
{
    d(c, &mut i, true);
}

// FIXME: This is a workaround, we should use `F` instead of `&mut F`
fn d<F>(c: char, i: &mut F, k: bool)
where
    F: FnMut(char),
{
    // 7-bit ASCII never decomposes
    if c <= '\x7f' {
        (*i)(c);
        return;
    }

    // Perform decomposition for Hangul
    if (c as u32) >= hangul::S_BASE && (c as u32) < (hangul::S_BASE + hangul::S_COUNT) {
        hangul::decompose(c, i);
        return;
    }

    // First check the canonical decompositions
    if let Some(canon) = canonical_decomposition(c) {
        for x in canon {
            d(*x, i, k);
        }
        return;
    }

    // Bottom out if we're not doing compat.
    if !k {
        (*i)(c);
        return;
    }

    // Then check the compatibility decompositions
    if let Some(compat) = compatibility_decomposition(c) {
        for x in compat {
            d(*x, i, k);
        }
        return;
    }

    // Finally bottom out.
    (*i)(c);
}

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
