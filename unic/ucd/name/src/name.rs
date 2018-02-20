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

pub static PREFIX_HANGUL_SYLLABLE: &'static str = "HANGUL SYLLABLE ";
pub static PREFIX_CJK_UNIFIED_IDEOGRAPH: &'static str = "CJK UNIFIED IDEOGRAPH-";
pub static PREFIX_TANGUT_IDEOGRAPH: &'static str = "TANGUT IDEOGRAPH-";
pub static PREFIX_NUSHU_CHARACTER: &'static str = "NUSHU CHARACTER-";
pub static PREFIX_CJK_COMPATIBILITY_IDEOGRAPH: &'static str = "CJK COMPATIBILITY IDEOGRAPH-";

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Name {
    NR1(char),
    NR2(&'static str, char),
    NR3(&'static [&'static str]),
}

#[cfg_attr(feature = "clippy", allow(len_without_is_empty))]
impl Name {
    pub fn of(ch: char) -> Option<Name> {
        use unic_char_property::tables::TCharDataTable;
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
        match self {
            &Name::NR1(ch) => {
                let mut len = PREFIX_HANGUL_SYLLABLE.len();
                // FIXME: use decomposed jamo short names instead
                len += Name::number_of_hex_digits(ch);
                len
            }
            &Name::NR2(prefix, ch) => {
                let mut len = prefix.len();
                len += Name::number_of_hex_digits(ch);
                len
            }
            &Name::NR3(pieces) => {
                // start with spaces
                let mut len = pieces.len().saturating_sub(1);
                for piece in pieces {
                    len += piece.len();
                }
                len
            }
        }
    }

    #[inline(always)]
    fn number_of_hex_digits(ch: char) -> usize {
        (32 - u32::leading_zeros(ch as u32) as usize + 3) / 4
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Name::NR1(ch) => {
                f.write_str(PREFIX_HANGUL_SYLLABLE)?;
                // FIXME: use decomposed jamo short names instead
                write!(f, "{:X}", ch as u32)
            }
            &Name::NR2(prefix, ch) => {
                f.write_str(prefix)?;
                write!(f, "{:X}", ch as u32)
            }
            &Name::NR3(pieces) => {
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
        match self {
            &Name::NR1(ch) => match other {
                &Name::NR1(other_ch) => ch.cmp(&other_ch),
                &Name::NR2(other_prefix, _) => PREFIX_HANGUL_SYLLABLE.cmp(&other_prefix),
                &Name::NR3(other_pieces) => {
                    let (first, _) = other_pieces.split_first().unwrap();
                    PREFIX_HANGUL_SYLLABLE.cmp(first)
                }
            },
            &Name::NR2(prefix, ch) => match other {
                &Name::NR1(_) => prefix.cmp(&PREFIX_HANGUL_SYLLABLE),
                &Name::NR2(other_prefix, other_ch) => {
                    if prefix == other_prefix {
                        ch.cmp(&other_ch)
                    } else {
                        prefix.cmp(&other_prefix)
                    }
                }
                &Name::NR3(other_pieces) => {
                    let (first, _) = other_pieces.split_first().unwrap();
                    prefix.cmp(first)
                }
            },
            &Name::NR3(pieces) => {
                let (first, _) = pieces.split_first().unwrap();
                match other {
                    &Name::NR1(_) => first.cmp(&PREFIX_HANGUL_SYLLABLE),
                    &Name::NR2(other_prefix, _) => first.cmp(&other_prefix),
                    &Name::NR3(other_pieces) => pieces.cmp(&other_pieces),
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
}
