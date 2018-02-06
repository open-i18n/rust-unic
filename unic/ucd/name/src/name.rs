// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::fmt;
use core::cmp::Ordering;

pub static PREFIX_HANGUL_SYLLABLE: &'static str = "HANGUL SYLLABLE ";
pub static PREFIX_CJK_UNIFIED_IDEOGRAPH: &'static str = "CJK UNIFIED IDEOGRAPH-";
pub static PREFIX_TANGUT_IDEOGRAPH: &'static str = "TANGUT IDEOGRAPH-";
pub static PREFIX_NUSHU_CHARACTER: &'static str = "NUSHU CHARACTER-";
pub static PREFIX_CJK_COMPATIBILITY_IDEOGRAPH: &'static str = "CJK COMPATIBILITY IDEOGRAPH-";

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Name {
    Pieces(&'static [&'static str]),
    HangulSyllable(char),
    CJKUnifiedIdeograph(char),
    TangutIdieograph(char),
    NushuCharacter(char),
    CJKCompatibilityIdeograph(char),
}

#[cfg_attr(feature = "clippy", allow(len_without_is_empty))]
impl Name {
    pub fn of(ch: char) -> Option<Name> {
        match ch {
            '\u{AC00}' ... '\u{D7A3}' => {
                Some(Name::HangulSyllable(ch))
            },
            '\u{3400}' ... '\u{4DB5}'
            | '\u{4E00}' ... '\u{9FEA}'
            | '\u{20000}' ... '\u{2A6D6}'
            | '\u{2A700}' ... '\u{2B734}'
            | '\u{2B740}' ... '\u{2B81D}'
            | '\u{2B820}' ... '\u{2CEA1}'
            | '\u{2CEB0}' ... '\u{2EBE0}' => {
                Some(Name::CJKUnifiedIdeograph(ch))
            },
            '\u{17000}' ... '\u{187EC}' => {
                Some(Name::TangutIdieograph(ch))
            },
            '\u{1B170}' ... '\u{1B2FB}' => {
                Some(Name::NushuCharacter(ch))
            },
            '\u{F900}' ... '\u{FA6D}'
            | '\u{FA70}' ... '\u{FAD9}'
            | '\u{2F800}' ... '\u{2FA1D}' => {
                Some(Name::CJKCompatibilityIdeograph(ch))
            },
            _ => {
                data::NAMES.find(ch).map(|pieces| Name::Pieces(pieces))
            },
        }
    }

    /// Length of the name in bytes.
    pub fn len(&self) -> usize {
        match self {
            &Name::Pieces(pieces) => {
                // start with spaces
                let mut len = pieces.len().saturating_sub(1);
                for piece in pieces {
                    len += piece.len();
                }
                len
            },
            &Name::HangulSyllable(ch) => {
                let mut len = PREFIX_HANGUL_SYLLABLE.len();
                // FIXME: use decomposed jamo short names instead
                len += self.num_of_hex_digits(ch as u32);
                len
            },
            &Name::CJKUnifiedIdeograph(ch) => {
                let mut len = PREFIX_CJK_UNIFIED_IDEOGRAPH.len();
                len += self.num_of_hex_digits(ch as u32);
                len
            },
            &Name::TangutIdieograph(ch) => {
                let mut len = PREFIX_TANGUT_IDEOGRAPH.len();
                len += self.num_of_hex_digits(ch as u32);
                len
            },
            &Name::NushuCharacter(ch) => {
                let mut len = PREFIX_NUSHU_CHARACTER.len();
                len += self.num_of_hex_digits(ch as u32);
                len
            },
            &Name::CJKCompatibilityIdeograph(ch) => {
                let mut len = PREFIX_CJK_COMPATIBILITY_IDEOGRAPH.len();
                len += self.num_of_hex_digits(ch as u32);
                len
            },
        }
    }

    fn num_of_hex_digits(&self, code_point: u32) -> usize {
        let mut num = code_point;
        let mut digits = 0;
        while num > 0 {
            num /= 16;
            digits += 1;
        }
        digits
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Name::Pieces(pieces) => {
                let (first, rest) = pieces.split_first().unwrap();
                f.write_str(first)?;
                for piece in rest {
                    f.write_str(" ")?;
                    f.write_str(piece)?;
                }
                Ok(())
            },
            &Name::HangulSyllable(ch) => {
                f.write_str(PREFIX_HANGUL_SYLLABLE)?;
                // FIXME: use decomposed jamo short names instead
                write!(f, "{:X}", ch as u32)
            },
            &Name::CJKUnifiedIdeograph(ch) => {
                f.write_str(PREFIX_CJK_UNIFIED_IDEOGRAPH)?;
                write!(f, "{:X}", ch as u32)
            },
            &Name::TangutIdieograph(ch) => {
                f.write_str(PREFIX_TANGUT_IDEOGRAPH)?;
                write!(f, "{:X}", ch as u32)
            },
            &Name::NushuCharacter(ch) => {
                f.write_str(PREFIX_NUSHU_CHARACTER)?;
                write!(f, "{:X}", ch as u32)
            },
            &Name::CJKCompatibilityIdeograph(ch) => {
                f.write_str(PREFIX_CJK_COMPATIBILITY_IDEOGRAPH)?;
                write!(f, "{:X}", ch as u32)
            },
        }
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Name) -> Ordering {
        match self {
            &Name::Pieces(pieces) => {
                let (first, _) = pieces.split_first().unwrap();
                match other {
                    &Name::Pieces(other_pieces) => pieces.cmp(other_pieces),
                    &Name::HangulSyllable(_) => first.cmp(&PREFIX_HANGUL_SYLLABLE),
                    &Name::CJKUnifiedIdeograph(_) => first.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::TangutIdieograph(_) => first.cmp(&PREFIX_TANGUT_IDEOGRAPH),
                    &Name::NushuCharacter(_) => first.cmp(&PREFIX_NUSHU_CHARACTER),
                    &Name::CJKCompatibilityIdeograph(_) => first.cmp(&PREFIX_CJK_COMPATIBILITY_IDEOGRAPH),
                }
            },
            &Name::HangulSyllable(ch) => {
                let prefix = PREFIX_HANGUL_SYLLABLE;
                match other {
                    &Name::Pieces(pieces) => {
                        let (first, _) = pieces.split_first().unwrap();
                        prefix.cmp(first)
                    },
                    &Name::HangulSyllable(other_ch) => ch.cmp(&other_ch),
                    &Name::CJKUnifiedIdeograph(_) => prefix.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::TangutIdieograph(_) => prefix.cmp(&PREFIX_TANGUT_IDEOGRAPH),
                    &Name::NushuCharacter(_) => prefix.cmp(&PREFIX_NUSHU_CHARACTER),
                    &Name::CJKCompatibilityIdeograph(_) => prefix.cmp(&PREFIX_CJK_COMPATIBILITY_IDEOGRAPH),
                }
            },
            &Name::CJKUnifiedIdeograph(ch) => {
                let prefix = PREFIX_CJK_UNIFIED_IDEOGRAPH;
                match other {
                    &Name::Pieces(pieces) => {
                        let (first, _) = pieces.split_first().unwrap();
                        prefix.cmp(first)
                    },
                    &Name::HangulSyllable(_) => prefix.cmp(&PREFIX_HANGUL_SYLLABLE),
                    &Name::CJKUnifiedIdeograph(other_ch) => ch.cmp(&other_ch),
                    &Name::TangutIdieograph(_) => prefix.cmp(&PREFIX_TANGUT_IDEOGRAPH),
                    &Name::NushuCharacter(_) => prefix.cmp(&PREFIX_NUSHU_CHARACTER),
                    &Name::CJKCompatibilityIdeograph(_) => prefix.cmp(&PREFIX_CJK_COMPATIBILITY_IDEOGRAPH),
                }
            },
            &Name::TangutIdieograph(ch) => {
                let prefix = PREFIX_TANGUT_IDEOGRAPH;
                match other {
                    &Name::Pieces(pieces) => {
                        let (first, _) = pieces.split_first().unwrap();
                        prefix.cmp(first)
                    },
                    &Name::HangulSyllable(_) => prefix.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::CJKUnifiedIdeograph(_) => prefix.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::TangutIdieograph(other_ch) => ch.cmp(&other_ch),
                    &Name::NushuCharacter(_) => prefix.cmp(&PREFIX_NUSHU_CHARACTER),
                    &Name::CJKCompatibilityIdeograph(_) => prefix.cmp(&PREFIX_CJK_COMPATIBILITY_IDEOGRAPH),
                }
            },
            &Name::NushuCharacter(ch) => {
                let prefix = PREFIX_NUSHU_CHARACTER;
                match other {
                    &Name::Pieces(pieces) => {
                        let (first, _) = pieces.split_first().unwrap();
                        prefix.cmp(first)
                    },
                    &Name::HangulSyllable(_) => prefix.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::CJKUnifiedIdeograph(_) => prefix.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::TangutIdieograph(_) => prefix.cmp(&PREFIX_TANGUT_IDEOGRAPH),
                    &Name::NushuCharacter(other_ch) => ch.cmp(&other_ch),
                    &Name::CJKCompatibilityIdeograph(_) => prefix.cmp(&PREFIX_CJK_COMPATIBILITY_IDEOGRAPH),
                }
            },
            &Name::CJKCompatibilityIdeograph(ch) => {
                let prefix = PREFIX_CJK_COMPATIBILITY_IDEOGRAPH;
                match other {
                    &Name::Pieces(pieces) => {
                        let (first, _) = pieces.split_first().unwrap();
                        prefix.cmp(first)
                    },
                    &Name::HangulSyllable(_) => prefix.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::CJKUnifiedIdeograph(_) => prefix.cmp(&PREFIX_CJK_UNIFIED_IDEOGRAPH),
                    &Name::TangutIdieograph(_) => prefix.cmp(&PREFIX_TANGUT_IDEOGRAPH),
                    &Name::NushuCharacter(_) => prefix.cmp(&PREFIX_NUSHU_CHARACTER),
                    &Name::CJKCompatibilityIdeograph(other_ch) => ch.cmp(&other_ch),
                }
            },
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
}
