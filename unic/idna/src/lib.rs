// Copyright 2016 The rust-url developers.
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

//! # UNIC — Unicode IDNA Compatibility Processing
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! This UNIC component implements algorithms from [Unicode Technical Standard #46 - Unicode IDNA
//! Compatibility Processing](http://unicode.org/reports/tr46/).
//!
//! Quoting from [UTS #46’s introduction](https://www.unicode.org/reports/tr46/#Introduction):
//!
//! > Initially, domain names were restricted to ASCII characters.
//! > A system was introduced in 2003 for internationalized domain names (IDN).
//! > This system is called Internationalizing Domain Names for Applications,
//! > or *IDNA2003* for short.
//! > This mechanism supports IDNs by means of a client software transformation
//! > into a format known as Punycode.
//! >
//! > A revision of IDNA was approved in 2010 (*IDNA2008*).
//! > This revision has a number of incompatibilities with IDNA2003.
//! >
//! > The incompatibilities force implementers of client software,
//! > such as browsers and emailers,
//! > to face difficult choices during the transition period
//! > as registries shift from IDNA2003 to IDNA2008.
//! > This document specifies a mechanism
//! > that minimizes the impact of this transition for client software,
//! > allowing client software to access domains that are valid under either system.

#[macro_use]
extern crate matches;

extern crate unic_normal;
extern crate unic_ucd_bidi;
extern crate unic_ucd_normal;

extern crate unic_idna_mapping as mapping;
extern crate unic_idna_punycode as punycode;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub use mapping::UNICODE_VERSION;

mod process;
pub use process::PUNYCODE_PREFIX;
pub use process::{to_ascii, to_unicode};
pub use process::{Errors, Flags};
