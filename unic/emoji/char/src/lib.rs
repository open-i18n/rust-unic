// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(
    bad_style,
    future_incompatible,
    missing_debug_implementations,
    unconditional_recursion,
    unsafe_code
)]
#![deny(missing_docs, unused)]

//! # UNIC — Unicode Emoji — Emoji Character Properties
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).

#[macro_use]
extern crate unic_char_property;
#[macro_use]
extern crate unic_char_range;
extern crate unic_ucd_version;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

mod emoji_version;
pub use emoji_version::{UnicodeVersion, EMOJI_VERSION};

mod emoji;
pub use emoji::{is_emoji, Emoji};

mod emoji_component;
pub use emoji_component::{is_emoji_component, EmojiComponent};

mod emoji_modifier;
pub use emoji_modifier::{is_emoji_modifier, EmojiModifier};

mod emoji_modifier_base;
pub use emoji_modifier_base::{is_emoji_modifier_base, EmojiModifierBase};

mod emoji_presentation;
pub use emoji_presentation::{is_emoji_presentation, EmojiPresentation};
