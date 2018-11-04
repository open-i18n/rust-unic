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
#![deny(unsafe_code)]

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
//! let sample_hangul = '갓';
//! assert!(is_syllable(sample_hangul));
//!
//! let mut decomposed = vec![];
//! {
//!     let mut collect_decomposed = |chr| {
//!         decomposed.push(chr);
//!     };
//!     decompose_syllable(sample_hangul, &mut collect_decomposed);
//! }
//! assert_eq!(decomposed, ['ᄀ', 'ᅡ', 'ᆺ']);
//!
//! let composed_lv = compose_syllable(decomposed[0], decomposed[1]).unwrap();
//! let composed = compose_syllable(composed_lv, decomposed[2]).unwrap();
//! assert_eq!(composed, sample_hangul);
//! ```

extern crate unic_ucd_version;
use unic_ucd_version::UnicodeVersion;

mod hangul;
pub use hangul::{compose_syllable, decompose_syllable, is_syllable};

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
