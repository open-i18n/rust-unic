// Copyright 2018 The UNIC Project Developers.
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

//! # UNIC — Unicode Character Tools — Basic Stable Character Properties
//!
//! A simple way to control iteration over a range of characters.
//!
//! # Examples
//!
//! ```
//! extern crate unic_char_basics;
//! use unic_char_basics::{is_noncharacter, is_private_use};
//!
//! # fn main() {
//! // Plane 0 (BMP)
//! assert_eq!(is_noncharacter('\u{0}'), false);
//! assert_eq!(is_noncharacter('\u{80}'), false);
//! assert_eq!(is_noncharacter('\u{e000}'), false);
//! assert_eq!(is_noncharacter('\u{f8ff}'), false);
//! assert_eq!(is_noncharacter('\u{fdd0}'), true);
//! assert_eq!(is_noncharacter('\u{fdef}'), true);
//! assert_eq!(is_noncharacter('\u{fffd}'), false);
//! assert_eq!(is_noncharacter('\u{fffe}'), true);
//! assert_eq!(is_noncharacter('\u{ffff}'), true);
//!
//! assert_eq!(is_private_use('\u{0}'), false);
//! assert_eq!(is_private_use('\u{80}'), false);
//! assert_eq!(is_private_use('\u{e000}'), true);
//! assert_eq!(is_private_use('\u{f8ff}'), true);
//! assert_eq!(is_private_use('\u{fdd0}'), false);
//! assert_eq!(is_private_use('\u{fdef}'), false);
//! assert_eq!(is_private_use('\u{fffd}'), false);
//! assert_eq!(is_private_use('\u{fffe}'), false);
//! assert_eq!(is_private_use('\u{ffff}'), false);
//!
//! // Plane 16 (PUA-B)
//! assert_eq!(is_noncharacter('\u{10_0000}'), false);
//! assert_eq!(is_noncharacter('\u{10_0001}'), false);
//! assert_eq!(is_noncharacter('\u{10_fffd}'), false);
//! assert_eq!(is_noncharacter('\u{10_fffe}'), true);
//! assert_eq!(is_noncharacter('\u{10_ffff}'), true);
//!
//! assert_eq!(is_private_use('\u{10_0000}'), true);
//! assert_eq!(is_private_use('\u{10_0001}'), true);
//! assert_eq!(is_private_use('\u{10_fffd}'), true);
//! assert_eq!(is_private_use('\u{10_fffe}'), false);
//! assert_eq!(is_private_use('\u{10_ffff}'), false);
//! # }
//! ```

#[cfg(feature = "std")]
extern crate core;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub mod noncharacter;
pub use noncharacter::is_noncharacter;

pub mod private_use;
pub use private_use::is_private_use;

pub mod notation;
pub use notation::unicode_notation;
