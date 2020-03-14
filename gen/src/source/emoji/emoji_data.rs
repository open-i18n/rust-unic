// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::char;
use std::collections::BTreeSet;
use std::str::FromStr;

use crate::source::ucd::BINARY_PROPERTIES_REGEX;
use crate::source::utils::read;

lazy_static! {
    pub static ref EMOJI_DATA: EmojiData = {
        read("external/unicode/ucd/data/emoji/emoji-data.txt")
            .parse()
            .unwrap()
    };
}

/// Emoji Character Properties
///
/// Ref: <https://www.unicode.org/reports/tr51/#Emoji_Properties>
#[derive(Clone, Debug, Default)]
pub struct EmojiData {
    /// Characters that are emoji.
    pub emoji: BTreeSet<char>,

    /// Characters that have emoji presentation by default.
    pub emoji_presentation: BTreeSet<char>,

    /// Characters that are emoji modifiers.
    pub emoji_modifier: BTreeSet<char>,

    /// Characters that can serve as a base for emoji modifiers.
    pub emoji_modifier_base: BTreeSet<char>,

    /// Characters that normally do not appear on emoji keyboards as separate choices, such as
    /// Keycap base characters, Regional_Indicators, ….
    pub emoji_component: BTreeSet<char>,

    /// Characters that are pictographic, or otherwise similar in kind to characters with the
    /// Emoji property.
    pub extended_pictographic: BTreeSet<char>,
}

impl FromStr for EmojiData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut props = EmojiData::default();

        for capture in BINARY_PROPERTIES_REGEX.captures_iter(str) {
            let low = char::from_u32(u32::from_str_radix(&capture[1], 16).unwrap()).unwrap();
            let high = capture
                .get(2)
                .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                .map(|u| char::from_u32(u).unwrap())
                .unwrap_or(low);
            let range = chars!(low..=high);

            match &capture[3] {
                "Emoji" => props.emoji.extend(range),
                "Emoji_Presentation" => props.emoji_presentation.extend(range),
                "Emoji_Modifier" => props.emoji_modifier.extend(range),
                "Emoji_Modifier_Base" => props.emoji_modifier_base.extend(range),
                "Emoji_Component" => props.emoji_component.extend(range),
                "Extended_Pictographic" => props.extended_pictographic.extend(range),
                prop => panic!("Unsupported EmojiData property `{}`", prop),
            }
        }

        Ok(props)
    }
}
