// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::cmp::Ordering;
use core::fmt;
use unic_ucd_hangul::decompose_syllable;

pub static PREFIX_HANGUL_SYLLABLE: &'static str = "HANGUL SYLLABLE ";
pub static PREFIX_CJK_UNIFIED_IDEOGRAPH: &'static str = "CJK UNIFIED IDEOGRAPH-";
pub static PREFIX_TANGUT_IDEOGRAPH: &'static str = "TANGUT IDEOGRAPH-";
pub static PREFIX_NUSHU_CHARACTER: &'static str = "NUSHU CHARACTER-";
pub static PREFIX_CJK_COMPATIBILITY_IDEOGRAPH: &'static str = "CJK COMPATIBILITY IDEOGRAPH-";

const JAMO_BUFFER_SIZE: usize = 3;

/// Represents values of the Unicode character property
/// [*Name*](https://www.unicode.org/reports/tr44/#Name).
///
/// Note: NR4 is omitted in this implementation because it can be represented by `None`.
///
/// See *Section 4.8* in [*Unicode*](http://www.unicode.org/versions/Unicode10.0.0/ch04.pdf)
/// for a full specification of all name derivation rules.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Name {
    /// NR1: For Hangul syllables, the Name is derived by rule,
    /// as specified in *Section 3.12* in
    /// [*Unicode*](http://www.unicode.org/versions/Unicode10.0.0/ch03.pdf)
    /// by concatenating a fixed prefix string "HANGUL SYLLABLE" and appropriate values of the
    /// [*Jamo_Short_Name*](http://www.unicode.org/Public/UCD/latest/ucd/Jamo.txt) property.
    NR1(char),

    /// NR2: For most ideographs, the Name is derived by
    /// concatenating a script-specific prefix string, as specified in
    /// [*Unicode*](http://www.unicode.org/versions/Unicode10.0.0/ch04.pdf),
    /// to the code point, expressed in hexadecimal, with the usual
    /// 4- to 6-digit convention.
    NR2(&'static str, char),

    /// NR3: For all other Graphic characters and for all Format characters,
    /// the Name is as explicitly listed in Field 1 of
    /// [*UnicodeData.txt*](https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt).
    NR3(&'static [&'static str]),
}

#[cfg_attr(feature = "cargo-clippy", allow(len_without_is_empty))]
impl Name {
    /// Find the character `Name` property value.
    pub fn of(ch: char) -> Option<Name> {
        match ch {
            '\u{AC00}'...'\u{D7A3}' => Some(Name::NR1(ch)),
            '\u{3400}'...'\u{4DB5}'
            | '\u{4E00}'...'\u{9FEA}'
            | '\u{20000}'...'\u{2A6D6}'
            | '\u{2A700}'...'\u{2B734}'
            | '\u{2B740}'...'\u{2B81D}'
            | '\u{2B820}'...'\u{2CEA1}'
            | '\u{2CEB0}'...'\u{2EBE0}' => Some(Name::NR2(PREFIX_CJK_UNIFIED_IDEOGRAPH, ch)),
            '\u{17000}'...'\u{187EC}' => Some(Name::NR2(PREFIX_TANGUT_IDEOGRAPH, ch)),
            '\u{1B170}'...'\u{1B2FB}' => Some(Name::NR2(PREFIX_NUSHU_CHARACTER, ch)),
            '\u{F900}'...'\u{FA6D}' | '\u{FA70}'...'\u{FAD9}' | '\u{2F800}'...'\u{2FA1D}' => {
                Some(Name::NR2(PREFIX_CJK_COMPATIBILITY_IDEOGRAPH, ch))
            }
            _ => data::NAMES.find(ch).map(|pieces| Name::NR3(pieces)),
        }
    }

    /// Length of the name in bytes.
    pub fn len(&self) -> usize {
        match *self {
            Name::NR1(ch) => {
                let mut len = PREFIX_HANGUL_SYLLABLE.len();
                {
                    let mut count_jamos = |jamo| {
                        let jamo_name = Name::jamo_short_name(jamo);
                        len += jamo_name.len();
                    };
                    decompose_syllable(ch, &mut count_jamos);
                }
                len
            }
            Name::NR2(prefix, ch) => {
                let mut len = prefix.len();
                len += Name::number_of_hex_digits(ch);
                len
            }
            Name::NR3(pieces) => {
                // start with spaces
                let mut len = pieces.len().saturating_sub(1);
                for piece in pieces {
                    len += piece.len();
                }
                len
            }
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
    #[inline(always)]
    fn number_of_hex_digits(ch: char) -> usize {
        (32 - u32::leading_zeros(ch as u32) as usize + 3) / 4
    }

    fn jamo_short_name(ch: char) -> &'static str {
        data::JAMO_SHORT_NAMES
            .find(ch)
            .expect("Unexpected jamo character")
    }

    fn collect_jamo_short_names(ch: char) -> [Option<&'static str>; JAMO_BUFFER_SIZE] {
        let mut jamos = [None; JAMO_BUFFER_SIZE];
        {
            let mut index = 0;
            let mut collect_jamos = |jamo| {
                debug_assert!(
                    index < JAMO_BUFFER_SIZE,
                    "Decomposed hangul jamos exceed buffer size limit",
                );
                jamos[index] = Some(Name::jamo_short_name(jamo));
                index += 1;
            };
            decompose_syllable(ch, &mut collect_jamos);
        }
        jamos
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Name::NR1(ch) => {
                f.write_str(PREFIX_HANGUL_SYLLABLE)?;
                let mut result = Ok(());
                {
                    let mut write_jamos = |jamo| {
                        let write_result = f.write_str(Name::jamo_short_name(jamo));
                        if write_result.is_err() {
                            result = write_result;
                        }
                    };
                    decompose_syllable(ch, &mut write_jamos);
                }
                result
            }
            Name::NR2(prefix, ch) => {
                f.write_str(prefix)?;
                write!(f, "{:X}", ch as u32)
            }
            Name::NR3(pieces) => {
                let (first, rest) = pieces.split_first().unwrap();
                f.write_str(first)?;
                for piece in rest {
                    f.write_str(" ")?;
                    f.write_str(piece)?;
                }
                Ok(())
            }
        }
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Name) -> Ordering {
        match *self {
            Name::NR1(ch) => match *other {
                Name::NR1(other_ch) => {
                    let jamos = Name::collect_jamo_short_names(ch);
                    let other_jamos = Name::collect_jamo_short_names(other_ch);
                    jamos.cmp(&other_jamos)
                }
                Name::NR2(other_prefix, _) => PREFIX_HANGUL_SYLLABLE.cmp(other_prefix),
                Name::NR3(other_pieces) => {
                    let (first, _) = other_pieces.split_first().unwrap();
                    PREFIX_HANGUL_SYLLABLE.cmp(first)
                }
            },
            Name::NR2(prefix, ch) => match *other {
                Name::NR1(_) => prefix.cmp(PREFIX_HANGUL_SYLLABLE),
                Name::NR2(other_prefix, other_ch) => {
                    if prefix == other_prefix {
                        ch.cmp(&other_ch)
                    } else {
                        prefix.cmp(other_prefix)
                    }
                }
                Name::NR3(other_pieces) => {
                    let (first, _) = other_pieces.split_first().unwrap();
                    prefix.cmp(first)
                }
            },
            Name::NR3(pieces) => {
                let (first, _) = pieces.split_first().unwrap();
                match *other {
                    Name::NR1(_) => first.cmp(&PREFIX_HANGUL_SYLLABLE),
                    Name::NR2(other_prefix, _) => first.cmp(&other_prefix),
                    Name::NR3(other_pieces) => pieces.cmp(other_pieces),
                }
            }
        }
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Name) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

mod data {
    use unic_char_property::tables::CharDataTable;
    include!("../tables/name_values.rsd");
    pub const NAMES: CharDataTable<&[&str]> = include!("../tables/name_map.rsv");
    pub const JAMO_SHORT_NAMES: CharDataTable<&str> = include!("../tables/jamo.rsv");
}
