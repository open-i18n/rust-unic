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

use reader::utils::read;

use super::BINARY_PROPERTIES_REGEX;


lazy_static! {
    pub static ref PROP_LIST: PropList = {
        read("data/ucd/PropList.txt").parse().unwrap()
    };
}


#[derive(Clone, Debug, Default)]
pub struct PropList {
    /// ASCII characters commonly used for the representation of hexadecimal numbers.
    ascii_hex_digit: BTreeSet<char>,

    /// Format control characters which have specific functions in the
    /// [Unicode Bidirectional Algorithm][UAX9].
    ///
    /// [UAX9]: http://unicode.org/reports/tr41/tr41-21.html#UAX9
    bidi_control: BTreeSet<char>,

    /// Punctuation characters explicitly called out as dashes in the Unicode Standard,
    /// plus their compatibility equivalents. Most of these have the `General_Category` value `Pd`,
    /// but some have the `General_Category` value `Sm` because of their use in mathematics.
    dash: BTreeSet<char>,

    /// For a machine-readable list of deprecated characters.
    /// No characters will ever be removed from the standard,
    /// but the usage of deprecated characters is strongly discouraged.
    deprecated: BTreeSet<char>,

    /// Characters that linguistically modify the meaning of another character to which they apply.
    /// Some diacritics are not combining characters,
    /// and some combining characters are not diacritics.
    diacritic: BTreeSet<char>,

    /// Characters whose principal function is to extend the value or shape of a preceding
    /// alphabetic character. Typical of these are length and iteration marks.
    extender: BTreeSet<char>,

    /// Characters commonly used for the representation of hexadecimal numbers,
    /// plus their compatibility equivalents.
    hex_digit: BTreeSet<char>,

    // hyphen is deprecated
    /// Characters considered to be CJKV (Chinese, Japanese, Korean, and Vietnamese)
    /// or other siniform (Chinese writing-related) ideographs.
    /// This property roughly defines the class of "Chinese characters" and does not
    /// include characters of other logographic scripts such as Cuneiform or Egyptian Hieroglyphs.
    /// The Ideographic property is used in the definition of Ideographic Description Sequences.
    ideographic: BTreeSet<char>,

    /// Used in Ideographic Description Sequences.
    ids_binary_operator: BTreeSet<char>,

    /// Used in Ideographic Description Sequences.
    ids_trinary_operator: BTreeSet<char>,

    /// Format control characters which have specific functions
    /// for control of cursive joining and ligation.
    join_control: BTreeSet<char>,

    /// A small number of spacing vowel letters occurring in certain Southeast Asian scripts
    /// such as Thai and Lao, which use a visual order display model.
    /// These letters are stored in text ahead of syllable-initial consonants,
    /// and require special handling for processes such as searching and sorting.
    logical_order_exception: BTreeSet<char>,

    /// Code points permanently reserved for internal use.
    noncharacter_code_point: BTreeSet<char>,

    /// Used in deriving the Alphabetic property.
    other_alphabetic: BTreeSet<char>,

    /// Used in deriving the `Default_Ignorable_Code_Point` property.
    other_default_ignorable_code_point: BTreeSet<char>,

    /// Used in deriving the `Grapheme_Extend` property.
    other_grapheme_extend: BTreeSet<char>,

    /// Used to maintain backward compatibility of `ID_Continue`.
    other_id_continue: BTreeSet<char>,

    /// Used to maintain backward compatibility of `ID_Start`.
    other_id_start: BTreeSet<char>,

    /// Used in deriving the `Lowercase` property.
    other_lowercase: BTreeSet<char>,

    /// Used in deriving the `Math` property.
    other_math: BTreeSet<char>,

    /// Used in deriving the `Uppercase` property.
    other_uppercase: BTreeSet<char>,

    /// Used for pattern syntax as described in
    /// Unicode Standard Annex #31, "Unicode Identifier and Pattern Syntax" ***[UAX31]***.
    ///
    /// [UAX31]: http://unicode.org/reports/tr41/tr41-21.html#UAX31
    pattern_syntax: BTreeSet<char>,

    /// Used for pattern syntax as described in
    /// Unicode Standard Annex #31, "Unicode Identifier and Pattern Syntax" ***[UAX31]***.
    ///
    /// [UAX31]: http://unicode.org/reports/tr41/tr41-21.html#UAX31
    pattern_white_space: BTreeSet<char>,

    /// A small class of visible format controls, which precede and then span a
    /// sequence of other characters, usually digits. These have also been known
    /// as "subtending marks", because most of them take a form which visually
    /// extends underneath the sequence of following digits.
    prepended_concatenation_mark: BTreeSet<char>,

    /// Punctuation characters that function as quotation marks.
    quotation_mark: BTreeSet<char>,

    /// Used in the definition of Ideographic Description Sequences.
    radical: BTreeSet<char>,

    /// Property of the regional indicator characters, U+1F1E6-1F1FF.
    /// This property is referenced in various segmentation algorithms,
    /// to assist in correct breaking around emoji flag sequences.
    regional_indicator: BTreeSet<char>,

    /// Punctuation characters that generally mark the end of sentences.
    /// Used in Unicode Standard Annex #29, "Unicode Text Segmentation" ***[UAX29]***.
    ///
    /// [UAX29]: http://unicode.org/reports/tr41/tr41-21.html#UAX29
    sentence_terminal: BTreeSet<char>,

    /// Characters with a "soft dot", like _i_ or _j_.
    /// An accent placed on these characters causes the dot to disappear.
    /// An explicit _dot above_ can be added where required, such as in Lithuanian.
    /// See Section 7.1, Latin in ***[Unicode]***.
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    soft_dotted: BTreeSet<char>,

    /// Punctuation characters that generally mark the end of textual units.
    terminal_punctuation: BTreeSet<char>,

    /// A property which specifies the exact set of Unified CJK Ideographs in the standard.
    /// This set excludes CJK Compatibility Ideographs (which have canonical decompositions to
    /// Unified CJK Ideographs), as well as characters from the CJK Symbols and Punctuation block.
    /// The class of `Unified_Ideograph=Y` characters is a proper subset
    /// of the class of `Ideographic=Y` characters.
    unified_ideograph: BTreeSet<char>,

    /// Indicates characters that are Variation Selectors.
    /// For details on the behavior of these characters,
    /// see _Section 23.4, Variation Selectors_ in [Unicode],
    /// and Unicode Technical Standard #37, "Unicode Ideographic Variation Database" [UTS37].
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    /// [UTS37]: http://unicode.org/reports/tr41/tr41-21.html#UTS37
    variation_selector: BTreeSet<char>,

    /// Spaces, separator characters and other control characters which should be treated
    /// by programming languages as "white space" for the purpose of parsing elements.
    /// See also `Line_Break`, `Grapheme_Cluster_Break`, `Sentence_Break`, and `Word_Break`,
    /// which classify space characters and related controls somewhat differently
    /// for particular text segmentation contexts.
    white_space: BTreeSet<char>,
}

impl FromStr for PropList {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut props = PropList::default();

        for capture in BINARY_PROPERTIES_REGEX.captures_iter(str) {
            let low = char::from_u32(u32::from_str_radix(&capture[1], 16).unwrap()).unwrap();
            let high = capture
                .get(2)
                .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                .map(|u| char::from_u32(u).unwrap())
                .unwrap_or(low);
            let range = chars!(low..=high);

            match &capture[3] {
                "White_Space" => props.white_space.extend(range),
                "Bidi_Control" => props.bidi_control.extend(range),
                "Join_Control" => props.join_control.extend(range),
                "Dash" => props.dash.extend(range),
                "Hyphen" => props.dash.extend(range),
                "Quotation_Mark" => props.quotation_mark.extend(range),
                "Terminal_Punctuation" => props.terminal_punctuation.extend(range),
                "Other_Math" => props.other_math.extend(range),
                "Hex_Digit" => props.hex_digit.extend(range),
                "ASCII_Hex_Digit" => props.ascii_hex_digit.extend(range),
                "Other_Alphabetic" => props.other_alphabetic.extend(range),
                "Ideographic" => props.ideographic.extend(range),
                "Diacritic" => props.diacritic.extend(range),
                "Extender" => props.extender.extend(range),
                "Other_Lowercase" => props.other_lowercase.extend(range),
                "Other_Uppercase" => props.other_uppercase.extend(range),
                "Noncharacter_Code_Point" => props.noncharacter_code_point.extend(range),
                "Other_Grapheme_Extend" => props.other_grapheme_extend.extend(range),
                "IDS_Binary_Operator" => props.ids_binary_operator.extend(range),
                "IDS_Trinary_Operator" => props.ids_trinary_operator.extend(range),
                "Radical" => props.radical.extend(range),
                "Unified_Ideograph" => props.unified_ideograph.extend(range),
                "Other_Default_Ignorable_Code_Point" => {
                    props.other_default_ignorable_code_point.extend(range)
                }
                "Deprecated" => props.deprecated.extend(range),
                "Soft_Dotted" => props.soft_dotted.extend(range),
                "Logical_Order_Exception" => props.logical_order_exception.extend(range),
                "Other_ID_Start" => props.other_id_start.extend(range),
                "Other_ID_Continue" => props.other_id_continue.extend(range),
                "Sentence_Terminal" => props.sentence_terminal.extend(range),
                "Variation_Selector" => props.variation_selector.extend(range),
                "Pattern_White_Space" => props.pattern_white_space.extend(range),
                "Pattern_Syntax" => props.pattern_syntax.extend(range),
                "Prepended_Concatenation_Mark" => props.prepended_concatenation_mark.extend(range),
                "Regional_Indicator" => props.regional_indicator.extend(range),
                prop => panic!("Unsupported PropList property `{}`", prop),
            }
        }

        Ok(props)
    }
}
