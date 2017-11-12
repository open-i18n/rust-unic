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


mod emoji;
mod emoji_component;
mod emoji_data_version;
mod emoji_modifier;
mod emoji_modifier_base;
mod emoji_presentation;
mod pkg_info;


pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub use emoji_data_version::EMOJI_DATA_VERSION;
pub use unic_ucd_version::{UnicodeVersion, UNICODE_VERSION};

pub use emoji::{is_emoji, Emoji};
pub use emoji_component::{is_emoji_component, EmojiComponent};
pub use emoji_modifier::{is_emoji_modifier, EmojiModifier};
pub use emoji_modifier_base::{is_emoji_modifier_base, EmojiModifierBase};
pub use emoji_presentation::{is_emoji_presentation, EmojiPresentation};
