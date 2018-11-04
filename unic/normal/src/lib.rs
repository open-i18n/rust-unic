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

#![warn(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    unconditional_recursion
)]
#![forbid(unsafe_code)]

//! # UNIC — Unicode Normalization Forms
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! This UNIC component implements algorithms from [Unicode Standard Annex #15 - Unicode
//! Normalization Forms](http://unicode.org/reports/tr15/).
//!
//! ```rust
//! extern crate unic_normal;
//!
//! use unic_normal::StrNormalForm;
//!
//! fn main() {
//!     let s = "ÅΩ";
//!     let c = s.nfc().collect::<String>();
//!     assert_eq!(c, "ÅΩ");
//! }
//! ```

extern crate unic_ucd_normal;

mod decompose;
mod recompose;

use std::str::Chars;

pub use decompose::Decompositions;
pub use recompose::Recompositions;
pub use unic_ucd_normal::UNICODE_VERSION;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

/// Methods for iterating over strings while applying Unicode normalizations
/// as described in
/// [Unicode Standard Annex #15](https://www.unicode.org/reports/tr15/).
pub trait StrNormalForm<I: Iterator<Item = char>> {
    /// Returns an iterator over the string in Unicode Normalization Form D
    /// (canonical decomposition).
    fn nfd(self) -> Decompositions<I>;

    /// Returns an iterator over the string in Unicode Normalization Form KD
    /// (compatibility decomposition).
    fn nfkd(self) -> Decompositions<I>;

    /// An Iterator over the string in Unicode Normalization Form C
    /// (canonical decomposition followed by canonical composition).
    fn nfc(self) -> Recompositions<I>;

    /// An Iterator over the string in Unicode Normalization Form KC
    /// (compatibility decomposition followed by canonical composition).
    fn nfkc(self) -> Recompositions<I>;
}

impl<'a> StrNormalForm<Chars<'a>> for &'a str {
    #[inline]
    fn nfd(self) -> Decompositions<Chars<'a>> {
        decompose::new_canonical(self.chars())
    }

    #[inline]
    fn nfkd(self) -> Decompositions<Chars<'a>> {
        decompose::new_compatible(self.chars())
    }

    #[inline]
    fn nfc(self) -> Recompositions<Chars<'a>> {
        recompose::new_canonical(self.chars())
    }

    #[inline]
    fn nfkc(self) -> Recompositions<Chars<'a>> {
        recompose::new_compatible(self.chars())
    }
}

impl<I: Iterator<Item = char>> StrNormalForm<I> for I {
    #[inline]
    fn nfd(self) -> Decompositions<I> {
        decompose::new_canonical(self)
    }

    #[inline]
    fn nfkd(self) -> Decompositions<I> {
        decompose::new_compatible(self)
    }

    #[inline]
    fn nfc(self) -> Recompositions<I> {
        recompose::new_canonical(self)
    }

    #[inline]
    fn nfkc(self) -> Recompositions<I> {
        recompose::new_compatible(self)
    }
}

#[cfg(test)]
mod tests {
    use super::StrNormalForm;

    #[test]
    fn test_nfd() {
        macro_rules! nfg_eq {
            ($input: expr, $expected: expr) => {
                assert_eq!($input.nfd().to_string(), $expected);
                // A dummy iterator that is not std::str::Chars directly;
                // note that `id_func` is used to ensure `Clone` implementation
                assert_eq!(
                    $input.chars().map(|c| c).nfd().collect::<String>(),
                    $expected
                );
            };
        }
        nfg_eq!("abc", "abc");
        nfg_eq!("\u{1e0b}\u{1c4}", "d\u{307}\u{1c4}");
        nfg_eq!("\u{2026}", "\u{2026}");
        nfg_eq!("\u{2126}", "\u{3a9}");
        nfg_eq!("\u{1e0b}\u{323}", "d\u{323}\u{307}");
        nfg_eq!("\u{1e0d}\u{307}", "d\u{323}\u{307}");
        nfg_eq!("a\u{301}", "a\u{301}");
        nfg_eq!("\u{301}a", "\u{301}a");
        nfg_eq!("\u{d4db}", "\u{1111}\u{1171}\u{11b6}");
        nfg_eq!("\u{ac1c}", "\u{1100}\u{1162}");
    }

    #[test]
    fn test_nfkd() {
        macro_rules! nfkd_eq {
            ($input: expr, $expected: expr) => {
                assert_eq!($input.nfkd().to_string(), $expected);
            };
        }
        nfkd_eq!("abc", "abc");
        nfkd_eq!("\u{1e0b}\u{1c4}", "d\u{307}DZ\u{30c}");
        nfkd_eq!("\u{2026}", "...");
        nfkd_eq!("\u{2126}", "\u{3a9}");
        nfkd_eq!("\u{1e0b}\u{323}", "d\u{323}\u{307}");
        nfkd_eq!("\u{1e0d}\u{307}", "d\u{323}\u{307}");
        nfkd_eq!("a\u{301}", "a\u{301}");
        nfkd_eq!("\u{301}a", "\u{301}a");
        nfkd_eq!("\u{d4db}", "\u{1111}\u{1171}\u{11b6}");
        nfkd_eq!("\u{ac1c}", "\u{1100}\u{1162}");
    }

    #[test]
    fn test_nfc() {
        macro_rules! nfc_eq {
            ($input: expr, $expected: expr) => {
                assert_eq!($input.nfc().to_string(), $expected);
            };
        }
        nfc_eq!("abc", "abc");
        nfc_eq!("\u{1e0b}\u{1c4}", "\u{1e0b}\u{1c4}");
        nfc_eq!("\u{2026}", "\u{2026}");
        nfc_eq!("\u{2126}", "\u{3a9}");
        nfc_eq!("\u{1e0b}\u{323}", "\u{1e0d}\u{307}");
        nfc_eq!("\u{1e0d}\u{307}", "\u{1e0d}\u{307}");
        nfc_eq!("a\u{301}", "\u{e1}");
        nfc_eq!("\u{301}a", "\u{301}a");
        nfc_eq!("\u{d4db}", "\u{d4db}");
        nfc_eq!("\u{ac1c}", "\u{ac1c}");
        nfc_eq!(
            "a\u{300}\u{305}\u{315}\u{5ae}b",
            "\u{e0}\u{5ae}\u{305}\u{315}b"
        );
    }

    #[test]
    fn test_nfkc() {
        macro_rules! nfkc_eq {
            ($input: expr, $expected: expr) => {
                assert_eq!($input.nfkc().to_string(), $expected);
            };
        }
        nfkc_eq!("abc", "abc");
        nfkc_eq!("\u{1e0b}\u{1c4}", "\u{1e0b}D\u{17d}");
        nfkc_eq!("\u{2026}", "...");
        nfkc_eq!("\u{2126}", "\u{3a9}");
        nfkc_eq!("\u{1e0b}\u{323}", "\u{1e0d}\u{307}");
        nfkc_eq!("\u{1e0d}\u{307}", "\u{1e0d}\u{307}");
        nfkc_eq!("a\u{301}", "\u{e1}");
        nfkc_eq!("\u{301}a", "\u{301}a");
        nfkc_eq!("\u{d4db}", "\u{d4db}");
        nfkc_eq!("\u{ac1c}", "\u{ac1c}");
        nfkc_eq!(
            "a\u{300}\u{305}\u{315}\u{5ae}b",
            "\u{e0}\u{5ae}\u{305}\u{315}b"
        );
    }
}
