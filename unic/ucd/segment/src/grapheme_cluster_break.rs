// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Unicode *Grapheme_Cluster_Break* Character Property.
//!
//! ## References
//!
//! * <http://www.unicode.org/reports/tr44/#Grapheme_Cluster_Break>
//! * <http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries>
//! * <http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Break_Property_Values>


use unic_char_property::PartialCharProperty;


char_property! {
    /// Represents the Unicode character
    /// [*Grapheme_Cluster_Break*](http://www.unicode.org/reports/tr44/#Grapheme_Cluster_Break)
    /// property.
    ///
    /// ## References
    ///
    /// * <http://www.unicode.org/reports/tr44/#Grapheme_Cluster_Break>
    /// * <http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries>
    /// * <http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Break_Property_Values>
    pub enum GraphemeClusterBreak {
        abbr => "GCB";
        long => "Grapheme_Cluster_Break";
        human => "Grapheme Cluster Break";

        /// ```text
        /// U+000D CARRIAGE RETURN (CR)
        /// ```
        CR {
            abbr => CR,
            long => CR,
            human => "Carriage Return",
        }

        /// ```text
        /// U+000A LINE FEED (LF)
        /// ```
        LF {
            abbr => LF,
            long => LF,
            human => "Line Feed",
        }

        /// ```text
        /// General_Category = Line_Separator, or
        /// General_Category = Paragraph_Separator, or
        /// General_Category = Control, or
        /// General_Category = Unassigned and Default_Ignorable_Code_Point, or
        /// General_Category = Surrogate, or
        /// General_Category = Format
        /// and not U+000D CARRIAGE RETURN
        /// and not U+000A LINE FEED
        /// and not U+200C ZERO WIDTH NON-JOINER (ZWNJ)
        /// and not U+200D ZERO WIDTH JOINER (ZWJ)
        /// ```
        Control {
            abbr => CN,
            long => Control,
            human => "Control",
        }

        /// ```text
        /// Grapheme_Extend = Yes
        ///
        /// This includes:
        /// General_Category = Nonspacing_Mark
        /// General_Category = Enclosing_Mark
        /// U+200C ZERO WIDTH NON-JOINER
        /// plus a few General_Category = Spacing_Mark needed for canonical equivalence.
        /// ```
        Extend {
            abbr => EX,
            long => Extend,
            human => "Extend",
        }

        /// ```text
        /// U+200D ZERO WIDTH JOINER
        /// ```
        ZWJ {
            abbr => ZWJ,
            long => ZWJ,
            human => "Zero Width Joiner (ZWJ)",
        }

        /// ```text
        /// Regional_Indicator = Yes
        /// ```
        ///
        /// This consists of the range:
        /// ```text
        /// U+1F1E6 REGIONAL INDICATOR SYMBOL LETTER A
        /// ..U+1F1FF REGIONAL INDICATOR SYMBOL LETTER Z
        /// ```
        RegionalIndicator {
            abbr => RI,
            long => Regional_Indicator,
            human => "Regional Indicator",
        }

        /// ```text
        /// Indic_Syllabic_Category = Consonant_Preceding_Repha, or
        /// Indic_Syllabic_Category = Consonant_Prefixed, or
        /// Prepended_Concatenation_Mark = Yes
        /// ```
        Prepend {
            abbr => PP,
            long => Prepend,
            human => "Prepend",
        }

        /// ```text
        /// Grapheme_Cluster_Break ≠ Extend, and
        /// General_Category = Spacing_Mark, or
        /// any of the following (which have General_Category = Other_Letter):
        /// U+0E33 ( ำ ) THAI CHARACTER SARA AM
        /// U+0EB3 ( ຳ ) LAO VOWEL SIGN AM
        /// ```
        ///
        /// Exceptions: The following (which have General_Category = Spacing_Mark and would
        /// otherwise be included) are specifically excluded:
        ///
        /// ```text
        /// U+102B ( ါ ) MYANMAR VOWEL SIGN TALL AA
        /// U+102C ( ာ ) MYANMAR VOWEL SIGN AA
        /// U+1038 ( း ) MYANMAR SIGN VISARGA
        /// U+1062 ( ၢ ) MYANMAR VOWEL SIGN SGAW KAREN EU
        /// ..U+1064 ( ၤ ) MYANMAR TONE MARK SGAW KAREN KE PHO
        /// U+1067 ( ၧ ) MYANMAR VOWEL SIGN WESTERN PWO KAREN EU
        /// ..U+106D ( ၭ ) MYANMAR SIGN WESTERN PWO KAREN TONE-5
        /// U+1083 ( ႃ ) MYANMAR VOWEL SIGN SHAN AA
        /// U+1087 ( ႇ ) MYANMAR SIGN SHAN TONE-2
        /// ..U+108C ( ႌ ) MYANMAR SIGN SHAN COUNCIL TONE-3
        /// U+108F ( ႏ ) MYANMAR SIGN RUMAI PALAUNG TONE-5
        /// U+109A ( ႚ ) MYANMAR SIGN KHAMTI TONE-1
        /// ..U+109C ( ႜ ) MYANMAR VOWEL SIGN AITON A
        /// U+1A61 ( ᩡ ) TAI THAM VOWEL SIGN A
        /// U+1A63 ( ᩣ ) TAI THAM VOWEL SIGN AA
        /// U+1A64 ( ᩤ ) TAI THAM VOWEL SIGN TALL AA
        /// U+AA7B ( ꩻ ) MYANMAR SIGN PAO KAREN TONE
        /// U+AA7D ( ꩽ ) MYANMAR SIGN TAI LAING TONE-5
        /// U+11720 ( 𑜠 ) AHOM VOWEL SIGN A
        /// U+11721 ( 𑜡 ) AHOM VOWEL SIGN AA
        /// ```
        SpacingMark {
            abbr => SM,
            long => SpacingMark,
            human => "Spacing Mark",
        }

        // Hangul

        /// ```text
        /// Hangul_Syllable_Type=L
        /// ```
        ///
        /// Such as:
        ///
        /// ```text
        /// U+1100 ( ᄀ ) HANGUL CHOSEONG KIYEOK
        /// U+115F ( ᅟ ) HANGUL CHOSEONG FILLER
        /// U+A960 ( ꥠ ) HANGUL CHOSEONG TIKEUT-MIEUM
        /// U+A97C ( ꥼ ) HANGUL CHOSEONG SSANGYEORINHIEUH
        /// ```
        L {
            abbr => L,
            long => L,
            human => "Hangul Syllable Type L",
        }

        /// ```text
        /// Hangul_Syllable_Type=V
        /// ```
        ///
        /// Such as:
        ///
        /// ```text
        /// U+1160 ( ᅠ ) HANGUL JUNGSEONG FILLER
        /// U+11A2 ( ᆢ ) HANGUL JUNGSEONG SSANGARAEA
        /// U+D7B0 ( ힰ ) HANGUL JUNGSEONG O-YEO
        /// U+D7C6 ( ퟆ ) HANGUL JUNGSEONG ARAEA-E
        /// ```
        V {
            abbr => V,
            long => V,
            human => "Hangul Syllable Type V",
        }

        /// ```text
        /// Hangul_Syllable_Type=T
        /// ```
        ///
        /// Such as:
        ///
        /// ```text
        /// U+11A8 ( ᆨ ) HANGUL JONGSEONG KIYEOK
        /// U+11F9 ( ᇹ ) HANGUL JONGSEONG YEORINHIEUH
        /// U+D7CB ( ퟋ ) HANGUL JONGSEONG NIEUN-RIEUL
        /// U+D7FB ( ퟻ ) HANGUL JONGSEONG PHIEUPH-THIEUTH
        /// ```
        T {
            abbr => T,
            long => T,
            human => "Hangul Syllable Type T",
        }

        /// ```text
        /// Hangul_Syllable_Type=LV:
        /// ```
        ///
        /// That is:
        ///
        /// ```text
        /// U+AC00 ( 가 ) HANGUL SYLLABLE GA
        /// U+AC1C ( 개 ) HANGUL SYLLABLE GAE
        /// U+AC38 ( 갸 ) HANGUL SYLLABLE GYA
        /// ...
        /// ```
        LV {
            abbr => LV,
            long => LV,
            human => "Hangul Syllable Type LV",
        }

        /// ```text
        /// Hangul_Syllable_Type=LVT
        /// ```
        ///
        /// That is:
        ///
        /// ```text
        /// U+AC01 ( 각 ) HANGUL SYLLABLE GAG
        /// U+AC02 ( 갂 ) HANGUL SYLLABLE GAGG
        /// U+AC03 ( 갃 ) HANGUL SYLLABLE GAGS
        /// U+AC04 ( 간 ) HANGUL SYLLABLE GAN
        /// ...
        /// ```
        LVT {
            abbr => LVT,
            long => LVT,
            human => "Hangul Syllable Type LVT",
        }

        // Emoji

        /// Emoji characters listed as `Emoji_Modifier_Base=Yes` in `emoji-data.txt`, which do not
        /// occur after ZWJ in `emoji-zwj-sequences.txt`.
        ///
        /// See <http://www.unicode.org/reports/tr51/>.
        EBase {
            abbr => EB,
            long => E_Base,
            human => "Emoji Base",
        }

        /// Emoji characters listed as `Emoji_Modifer=Yes` in `emoji-data.txt`.
        ///
        /// See <http://www.unicode.org/reports/tr51/>.
        EModifier {
            abbr => EM,
            long => E_Modifier,
            human => "Emoji Modifier",
        }

        /// Emoji characters that do not break from a previous ZWJ in a defined emoji ZWJ sequence,
        /// and are not listed as `Emoji_Modifier_Base=Yes` in `emoji-data.txt`.
        ///
        /// See <http://www.unicode.org/reports/tr51/>.
        GlueAfterZwj {
            abbr => GAZ,
            long => Glue_After_Zwj,
            human => "Glue After ZWJ",
        }

        /// Emoji characters listed as `Emoji_Modifer_Base=Yes` in `emoji_data.txt`, and also occur
        /// after ZWJ in `emoji-zwj-sequences.txt`.
        ///
        /// See <http://www.unicode.org/reports/tr51/>.
        EBaseGAZ {
            abbr => EBG,
            long => E_Base_GAZ,
            human => "Emoji Base and Glue After ZWJ",
        }

    }

    /// Abbreviated name aliases for the
    /// [*Grapheme_Cluster_Break*](http://www.unicode.org/reports/tr44/#Grapheme_Cluster_Break)
    /// property.
    ///
    /// ## See Also
    ///
    /// * <http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries>
    pub mod abbr_names for abbr;

    /// Long name aliases for the
    /// [*Grapheme_Cluster_Break*](http://www.unicode.org/reports/tr44/#Grapheme_Cluster_Break)
    /// property.
    ///
    /// ## See Also
    ///
    /// * <http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries>
    pub mod long_names for long;
}


impl PartialCharProperty for GraphemeClusterBreak {
    fn of(ch: char) -> Option<Self> {
        Self::of(ch)
    }
}


mod data {
    use super::long_names as GCB;
    use unic_utils::CharDataTable;
    pub const GRAPHEME_CLUSTER_BREAK_TABLE: CharDataTable<super::GraphemeClusterBreak> =
        include!("../tables/grapheme_cluster_break.rsv");
}


impl GraphemeClusterBreak {
    /// Find the character *Grapheme_Cluster_Break* property value.
    pub fn of(ch: char) -> Option<GraphemeClusterBreak> {
        data::GRAPHEME_CLUSTER_BREAK_TABLE.find(ch)
    }
}



#[cfg(test)]
mod tests {
    use unic_char_property::EnumeratedCharProperty;
    use super::GraphemeClusterBreak as GCB;

    #[test]
    fn test_ascii() {
        assert_eq!(GCB::of('\u{0000}'), Some(GCB::Control));
        assert_eq!(GCB::of('\u{0040}'), None);
        assert_eq!(GCB::of('\u{0041}'), None);
        assert_eq!(GCB::of('\u{0062}'), None);
        assert_eq!(GCB::of('\u{007F}'), Some(GCB::Control));
    }

    #[test]
    fn test_bmp() {
        // Hebrew
        assert_eq!(GCB::of('\u{0590}'), None);
        assert_eq!(GCB::of('\u{05D0}'), None);
        assert_eq!(GCB::of('\u{05D1}'), None);
        assert_eq!(GCB::of('\u{05FF}'), None);

        // Arabic
        assert_eq!(GCB::of('\u{0600}'), Some(GCB::Prepend));
        assert_eq!(GCB::of('\u{0627}'), None);
        assert_eq!(GCB::of('\u{07BF}'), None);

        // Default R + Arabic Extras
        assert_eq!(GCB::of('\u{07C0}'), None);
        assert_eq!(GCB::of('\u{085F}'), None);
        assert_eq!(GCB::of('\u{0860}'), None);
        assert_eq!(GCB::of('\u{0870}'), None);
        assert_eq!(GCB::of('\u{089F}'), None);
        assert_eq!(GCB::of('\u{08A0}'), None);
        assert_eq!(GCB::of('\u{089F}'), None);
        assert_eq!(GCB::of('\u{08FF}'), Some(GCB::Extend));

        // Default ET
        assert_eq!(GCB::of('\u{20A0}'), None);
        assert_eq!(GCB::of('\u{20CF}'), None);

        // Arabic Presentation Forms
        assert_eq!(GCB::of('\u{FB1D}'), None);
        assert_eq!(GCB::of('\u{FB4F}'), None);
        assert_eq!(GCB::of('\u{FB50}'), None);
        assert_eq!(GCB::of('\u{FDCF}'), None);
        assert_eq!(GCB::of('\u{FDF0}'), None);
        assert_eq!(GCB::of('\u{FDFF}'), None);
        assert_eq!(GCB::of('\u{FE70}'), None);
        assert_eq!(GCB::of('\u{FEFE}'), None);
        assert_eq!(GCB::of('\u{FEFF}'), Some(GCB::Control));

        // noncharacters
        assert_eq!(GCB::of('\u{FDD0}'), None);
        assert_eq!(GCB::of('\u{FDD1}'), None);
        assert_eq!(GCB::of('\u{FDEE}'), None);
        assert_eq!(GCB::of('\u{FDEF}'), None);
        assert_eq!(GCB::of('\u{FFFE}'), None);
        assert_eq!(GCB::of('\u{FFFF}'), None);
    }

    #[test]
    fn test_smp() {
        // Default AL + R
        assert_eq!(GCB::of('\u{10800}'), None);
        assert_eq!(GCB::of('\u{10FFF}'), None);
        assert_eq!(GCB::of('\u{1E800}'), None);
        assert_eq!(GCB::of('\u{1EDFF}'), None);
        assert_eq!(GCB::of('\u{1EE00}'), None);
        assert_eq!(GCB::of('\u{1EEFF}'), None);
        assert_eq!(GCB::of('\u{1EF00}'), None);
        assert_eq!(GCB::of('\u{1EFFF}'), None);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(GCB::of('\u{30000}'), None);
        assert_eq!(GCB::of('\u{40000}'), None);
        assert_eq!(GCB::of('\u{50000}'), None);
        assert_eq!(GCB::of('\u{60000}'), None);
        assert_eq!(GCB::of('\u{70000}'), None);
        assert_eq!(GCB::of('\u{80000}'), None);
        assert_eq!(GCB::of('\u{90000}'), None);
        assert_eq!(GCB::of('\u{a0000}'), None);
    }

    #[test]
    fn test_abbr_name() {
        assert_eq!(GCB::CR.abbr_name(), "CR");
    }

    #[test]
    fn test_long_name() {
        assert_eq!(GCB::CR.long_name(), "CR");
    }

    #[test]
    fn test_human_name() {
        assert_eq!(GCB::CR.human_name(), "Carriage Return");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", GCB::CR), "Carriage Return");
    }
}
