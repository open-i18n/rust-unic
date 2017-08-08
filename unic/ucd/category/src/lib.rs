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

//! # UNIC — UCD — Category
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Unicode [*General_Category*](http://unicode.org/reports/tr44/#General_Category).
//!
//! > The *General_Category* property of a code point provides for the most general classification
//! of that code point. It is usually determined based on the primary characteristic of the assigned
//! character for that code point. For example, is the character a letter, a mark, a number,
//! punctuation, or a symbol, and if so, of what type? Other *General_Category* values define the
//! classification of code points which are not assigned to regular graphic characters, including
//! such statuses as private-use, control, surrogate code point, and reserved unassigned.
//!
//! > Many characters have multiple uses, and not all such cases can be captured entirely by the
//! *General_Category* value. For example, the *General_Category* value of Latin, Greek, or Hebrew
//! letters does not attempt to cover (or preclude) the numerical use of such letters as Roman
//! numerals or in other numerary systems. Conversely, the *General_Category* of ASCII digits 0..9
//! as Nd (decimal digit) neither attempts to cover (or preclude) the occasional use of these digits
//! as letters in various orthographies. The *General_Category* is simply the first-order, most
//! usual categorization of a character.
//!
//! > For more information about the *General_Category* property, see Chapter 4, Character
//! Properties in [*Unicode*](http://unicode.org/reports/tr41/tr41-21.html#Unicode).
//!
//! -- [Unicode® Standard Annex #44 - Unicode Character Database](http://unicode.org/reports/tr44/)

#[macro_use]
extern crate matches;
extern crate unic_ucd_core;
extern crate unic_utils;

mod category;

pub use category::GeneralCategory;

use unic_ucd_core::UnicodeVersion;

/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("tables/unicode_version.rsv");
