// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::Path;

use source::emoji::emoji_data::EMOJI_DATA;
use source::emoji::readme::{EmojiDataVersion, EMOJI_VERSION};

use writer::utils::tables::ToRangeCharSet;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_emoji_data_version(dir, &EMOJI_VERSION);
    emit_emoji(dir);
    emit_emoji_presentation(dir);
    emit_emoji_modifier(dir);
    emit_emoji_modifier_base(dir);
    emit_emoji_component(dir);
    emit_extended_pictographic(dir);
}

pub fn emit_emoji_data_version(dir: &Path, emoji_version: &EmojiDataVersion) {
    write(
        dir,
        "emoji_version.rsv",
        &format!(
            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
            emoji_version.major, emoji_version.minor, emoji_version.micro,
        ),
    );
}

pub fn emit_emoji(dir: &Path) {
    write(dir, "emoji.rsv", &EMOJI_DATA.emoji.to_range_char_set());
}

pub fn emit_emoji_presentation(dir: &Path) {
    write(
        dir,
        "emoji_presentation.rsv",
        &EMOJI_DATA.emoji_presentation.to_range_char_set(),
    );
}

pub fn emit_emoji_modifier(dir: &Path) {
    write(
        dir,
        "emoji_modifier.rsv",
        &EMOJI_DATA.emoji_modifier.to_range_char_set(),
    );
}

pub fn emit_emoji_modifier_base(dir: &Path) {
    write(
        dir,
        "emoji_modifier_base.rsv",
        &EMOJI_DATA.emoji_modifier_base.to_range_char_set(),
    );
}

pub fn emit_emoji_component(dir: &Path) {
    write(
        dir,
        "emoji_component.rsv",
        &EMOJI_DATA.emoji_component.to_range_char_set(),
    );
}

pub fn emit_extended_pictographic(dir: &Path) {
    write(
        dir,
        "extended_pictographic.rsv",
        &EMOJI_DATA.extended_pictographic.to_range_char_set(),
    );
}
