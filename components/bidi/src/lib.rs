// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code, missing_docs)]

//! # UNIC — Unicode Bidirectional Algorithm
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! This UNIC component implements algorithms from [Unicode Standard Annex #9 - Unicode
//! Bidirectional Algorithm](http://unicode.org/reports/tr9/), a.k.a.  *UBA*, used for display of
//! mixed right-to-left and left-to-right text.  It is written in safe Rust, compatible with the
//! current stable release.
//!
//!
//! ## Example
//!
//! ```rust
//! use unic_bidi::BidiInfo;
//!
//! // This example text is defined using `concat!` because some browsers
//! // and text editors have trouble displaying bidi strings.
//! let text = concat![
//!   "א",
//!   "ב",
//!   "ג",
//!   "a",
//!   "b",
//!   "c",
//! ];
//!
//! // Resolve embedding levels within the text.  Pass `None` to detect the
//! // paragraph level automatically.
//! let bidi_info = BidiInfo::new(&text, None);
//!
//! // This paragraph has embedding level 1 because its first strong character is RTL.
//! assert_eq!(bidi_info.paragraphs.len(), 1);
//! let para = &bidi_info.paragraphs[0];
//! assert_eq!(para.level.number(), 1);
//! assert_eq!(para.level.is_rtl(), true);
//!
//! // Re-ordering is done after wrapping each paragraph into a sequence of
//! // lines. For this example, I'll just use a single line that spans the
//! // entire paragraph.
//! let line = para.range.clone();
//!
//! let display = bidi_info.reorder_line(para, line);
//! assert_eq!(display, concat![
//!   "a",
//!   "b",
//!   "c",
//!   "ג",
//!   "ב",
//!   "א",
//! ]);
//! ```
//!
//! [tr9]: http://www.unicode.org/reports/tr9/


extern crate unic_ucd_bidi;

#[macro_use]
extern crate matches;

#[cfg(feature = "with_serde")]
#[macro_use]
extern crate serde_derive;

#[cfg(all(feature = "with_serde", test))]
extern crate serde_test;

pub mod format_chars;
pub mod level;

mod explicit;
mod implicit;
mod prepare;
mod process;


pub use unic_ucd_bidi::{BidiClass, bidi_class, UNICODE_VERSION};
pub use level::Level;
pub use prepare::LevelRun;
pub use process::{ParagraphInfo, BidiInfo};


/// UNIC component version.
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// UNIC component name.
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

/// UNIC component description.
pub const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
