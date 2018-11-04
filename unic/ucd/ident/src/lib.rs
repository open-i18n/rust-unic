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
#![forbid(unsafe_code)]

//! # UNIC — UCD — Identifier Properties
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor to the UCD properties used widely by [UAX31 Unicode Identifier and Pattern Syntax].
//!
//! # Features
//!
//! - `xid` (default): the `XID_Start` and `XID_Continue` properties
//! - `id` (optional): the `ID_Start` and `ID_Continue` properties;
//!   _in most cases, you should prefer using the `XID` properties
//!   because they are consistent under NFKC normalization_
//! - `pattern` (optional): the `Pattern_Syntax` and `Pattern_White_Space` properties
//!
//! [UAX31 Unicode Identifier and Pattern Syntax]: <https://www.unicode.org/reports/tr31/>

#[macro_use]
extern crate unic_char_property;
#[macro_use]
extern crate unic_char_range;
extern crate unic_ucd_version;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

#[cfg(feature = "xid")]
mod xid {
    char_property! {
        /// A character that can start an identifier, stable under NFKC.
        pub struct XidStart(bool) {
            abbr => "XIDS";
            long => "XID_Start";
            human => "XID Start";

            data_table_path => "../tables/xid_start.rsv";
        }

        /// Is this a NFKC-safe identifier starting character?
        pub fn is_xid_start(char) -> bool;
    }
    char_property! {
        /// A character that can continue an identifier, stable under NFKC.
        pub struct XidContinue(bool) {
            abbr => "XIDC";
            long => "XID_Continue";
            human => "XID Continue";

            data_table_path => "../tables/xid_continue.rsv";
        }

        /// Is this a NFKC-safe identifier continuing character?
        pub fn is_xid_continue(char) -> bool;
    }
}
#[cfg(feature = "xid")]
pub use xid::{is_xid_continue, is_xid_start, XidContinue, XidStart};

#[cfg(feature = "id")]
mod id {
    char_property! {
        /// A character that can start an identifier.
        pub struct IdStart(bool) {
            abbr => "IDS";
            long => "ID_Start";
            human => "ID Start";

            data_table_path => "../tables/id_start.rsv";
        }

        /// Is this a identifier starting character?
        pub fn is_id_start(char) -> bool;
    }
    char_property! {
        /// A character that can continue an identifier.
        pub struct IdContinue(bool) {
            abbr => "IDC";
            long => "ID_Continue";
            human => "ID Continue";

            data_table_path => "../tables/id_continue.rsv";
        }

        /// Is this a identifier continuing character?
        pub fn is_id_continue(char) -> bool;
    }
}
#[cfg(feature = "id")]
pub use id::{is_id_continue, is_id_start, IdContinue, IdStart};

#[cfg(feature = "pattern")]
mod pattern {
    char_property! {
        /// A character that should be treated as a syntax character in patterns.
        pub struct PatternSyntax(bool) {
            abbr => "Pat_Syn";
            long => "Pattern_Syntax";
            human => "Pattern Syntax";

            data_table_path => "../tables/pattern_syntax.rsv";
        }

        /// Is this a character that should be treated as syntax in patterns?
        pub fn is_pattern_syntax(char) -> bool;
    }
    char_property! {
        /// A character that should be treated as a whitespace in patterns.
        pub struct PatternWhitespace(bool) {
            abbr => "Pat_WS";
            long => "Pattern_White_Space";
            human => "Pattern Whitespace";

            data_table_path => "../tables/pattern_syntax.rsv";
        }

        /// Is this a character that should be treated as whitespace in patterns?
        pub fn is_pattern_whitespace(char) -> bool;
    }
}
#[cfg(feature = "pattern")]
pub use pattern::{is_pattern_syntax, is_pattern_whitespace, PatternSyntax, PatternWhitespace};

use unic_ucd_version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
