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
#![forbid(future_incompatible, missing_debug_implementations, unconditional_recursion, unsafe_code)]
#![deny(bad_style, unsafe_code, unused)]

//! # UNIC - UCD - Hangul Syllable Composition & Decomposition
//!
//! This UCD component provides algorithms described in [Unicode®
//! Standard - 3.12 Conjoining Jamo Behavior](https://www.unicode.org/versions/latest/ch03.pdf),
//! used for detecting, composing and decomposing Hangul syllables.
//!
//! # Examples
//!
//! ```rust
//! # use unic_ucd_hangul::{is_syllable, compose_syllable, decompose_syllable};
//!
//! assert!(is_syllable('갏'));
//! assert!(!is_syllable('A'));
//!
//! let mut decomposed_lv = vec![];
//! {
//!     let mut collect_decomposed = |chr| {
//!         decomposed_lv.push(chr);
//!     };
//!     decompose_syllable('쮀', &mut collect_decomposed);
//! }
//! assert_eq!(decomposed_lv, ['ᄍ', 'ᅰ']);
//!
//! assert_eq!(
//!     compose_syllable('ᄑ', 'ᅱ').unwrap(),
//!     '퓌',
//! );
//! ```

extern crate unic_ucd_version;
use unic_ucd_version::UnicodeVersion;

mod hangul;
pub use hangul::{is_syllable, compose_syllable, decompose_syllable};

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
