// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Functions for computing canonical and compatible decompositions for Unicode characters.

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


mod tables;
mod hangul;

pub use tables::UNICODE_VERSION;
pub use tables::{canonical_decomposition, compatibility_decomposition, composition,
                 canonical_combining_class};
pub use tables::is_combining_mark;

use std::cmp::Ordering::{Equal, Less, Greater};
use std::ops::FnMut;


/// Compute canonical Unicode decomposition for character.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
pub fn decompose_canonical<F>(c: char, mut i: F)
    where F: FnMut(char)
{
    d(c, &mut i, false);
}

/// Compute canonical or compatible Unicode decomposition for character.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
pub fn decompose_compatible<F>(c: char, mut i: F)
    where F: FnMut(char)
{
    d(c, &mut i, true);
}

// FIXME: This is a workaround, we should use `F` instead of `&mut F`
fn d<F>(c: char, i: &mut F, k: bool)
    where F: FnMut(char)
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
    match canonical_decomposition(c) {
        Some(canon) => {
            for x in canon {
                d(*x, i, k);
            }
            return;
        }
        None => (),
    }

    // Bottom out if we're not doing compat.
    if !k {
        (*i)(c);
        return;
    }

    // Then check the compatibility decompositions
    match compatibility_decomposition(c) {
        Some(compat) => {
            for x in compat {
                d(*x, i, k);
            }
            return;
        }
        None => (),
    }

    // Finally bottom out.
    (*i)(c);
}

/// Compose two characters into a single character, if possible.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
pub fn compose(a: char, b: char) -> Option<char> {
    hangul::compose(a, b).or_else(
        || match composition(a) {
            None => None,
            Some(candidates) => {
                match candidates.binary_search_by(
                    |&(val, _)| if b == val {
                        Equal
                    } else if val < b {
                        Less
                    } else {
                        Greater
                    }
                ) {
                    Ok(idx) => {
                        let (_, result) = candidates[idx];
                        Some(result)
                    }
                    Err(_) => None,
                }
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use std::char;

    use super::*;

    #[test]
    fn test_is_combining_mark_ascii() {
        for cp in 0..0x7f {
            assert!(!is_combining_mark(char::from_u32(cp).unwrap()));
        }
    }

    // TODO: Add more tests for edge cases, Hangul comp/decomp, etc

    #[test]
    fn test_is_combining_mark_misc() {
        // https://github.com/unicode-rs/unicode-normalization/issues/16
        // U+11C3A BHAIKSUKI VOWEL SIGN O
        // Category: Mark, Nonspacing [Mn]
        assert!(is_combining_mark('\u{11C3A}'));

        // U+11C3F BHAIKSUKI SIGN VIRAMA
        // Category: Mark, Nonspacing [Mn]
        assert!(is_combining_mark('\u{11C3F}'));
    }
}
