// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code, unconditional_recursion)]
#![deny(missing_docs)]

//! # UNIC — Unicode Emoji — Emoji Character Properties
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).

#[macro_use]
extern crate unic_char_property;

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd_version;


pub mod emoji;
pub mod emoji_presentation;
pub mod emoji_modifier;
pub mod emoji_modifier_base;
pub mod emoji_component;


pub use emoji::{is_emoji, Emoji};
pub use emoji_presentation::{is_emoji_presentation, EmojiPresentation};
pub use emoji_modifier::{is_emoji_modifier, EmojiModifier};
pub use emoji_modifier_base::{is_emoji_modifier_base, EmojiModifierBase};
pub use emoji_component::{is_emoji_component, EmojiComponent};


/* TODO: Figure out relation between UNICODE_VERSION and EMOJI_VERSION
use unic_ucd_version::UnicodeVersion;


/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");
*/
