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
use std::str::FromStr;

use regex::Regex;

use source::utils::read;

lazy_static! {
    pub static ref UNICODE_DATA: UnicodeData =
        { read("data/ucd/UnicodeData.txt").parse().unwrap() };
}

/// Data line from UnicodeData.txt
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnicodeDataEntry {
    pub character: char,

    /// (1) When a string value not enclosed in &lt;angle brackets> occurs in this field,
    /// it specifies the character's Name property value,
    /// which matches exactly the name published in the code charts.
    /// The Name property value for most ideographic characters and for Hangul syllables
    /// is derived instead by various rules.
    /// See *Section 4.8, Name* in *[Unicode]* for a full specification of those rules.
    /// Strings enclosed in &lt;angle brackets> in this field either provide
    /// label information used in the name derivation rules,
    /// or—in the case of characters which have a null string as their Name property value,
    /// such as control characters—provide other information about their code point type.
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub name: String,

    /// (2) This is a useful breakdown into various character types which
    /// can be used as a default categorization in implementations.
    /// For the property values, see [General_Category Values].
    ///
    /// [General_Category Values]: http://unicode.org/reports/tr44/#General_Category_Values
    pub general_category: String,

    /// (3) The classes used for the Canonical Ordering Algorithm in the Unicode Standard.
    /// This property could be considered either an enumerated property or a numeric property:
    /// the principal use of the property is in terms of the numeric values.
    /// For the property value names associated with different numeric values,
    /// see [DerivedCombiningClass.txt] and [Canonical_Combining_Class Values][CCC Values].
    ///
    /// [DerivedCombiningClass.txt]: http://unicode.org/reports/tr44/#DerivedCombiningClass.txt
    /// [CCC Values]: http://unicode.org/reports/tr44/#Canonical_Combining_Class_Values
    pub canonical_combining_class: u8,

    /// (4) These are the categories required by the Unicode Bidirectional Algorithm.
    /// For the property values, see [Bidirectional Class Values].
    /// For more information, see Unicode Standard Annex #9,
    /// "Unicode Bidirectional Algorithm" *[UAX9]*.
    ///
    /// The default property values depend on the code point,
    /// and are explained in DerivedBidiClass.txt
    ///
    /// [Bidirectional Class Values]: http://unicode.org/reports/tr44/#Bidi_Class_Values
    /// [UAX9]: http://unicode.org/reports/tr41/tr41-21.html#UAX9
    pub bidi_class: String,

    /// (5) This is one half of the field containing both the values
    /// [`Decomposition_Type` and `Decomposition_Mapping`], with the type in angle brackets.
    /// The decomposition mappings exactly match the decomposition mappings
    /// published with the character names in the Unicode Standard.
    /// For more information, see [Character Decomposition Mappings][Decomposition Mappings].
    ///
    /// [Decomposition Mappings]: http://unicode.org/reports/tr44/#Character_Decomposition_Mappings
    pub decomposition_type: Option<String>,

    /// (5) This is one half of the field containing both the values
    /// [`Decomposition_Type` and `Decomposition_Mapping`], with the type in angle brackets.
    /// The decomposition mappings exactly match the decomposition mappings
    /// published with the character names in the Unicode Standard.
    /// For more information, see [Character Decomposition Mappings][Decomposition Mappings].
    ///
    /// [Decomposition Mappings]: http://unicode.org/reports/tr44/#Character_Decomposition_Mappings
    pub decomposition_mapping: Option<Box<[char]>>,

    /// (6) If the character has the property value `Numeric_Type=Decimal`,
    /// then the `Numeric_Value` of that digit is represented with an integer value
    /// (limited to the range 0..9) in fields 6, 7, and 8.
    /// Characters with the property value `Numeric_Type=Decimal` are restricted to digits
    /// which can be used in a decimal radix positional numeral system and
    /// which are encoded in the standard in a contiguous ascending range 0..9.
    /// See the discussion of *decimal digits* in *Chapter 4, Character Properties* in *[Unicode]*.
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub decimal_numeric_value: Option<u8>,

    /// (7) If the character has the property value `Numeric_Type=Digit`,
    /// then the `Numeric_Value` of that digit is represented with an integer value
    /// (limited to the range 0..9) in fields 7 and 8, and field 6 is null.
    /// This covers digits that need special handling, such as the compatibility superscript digits.
    ///
    /// Starting with Unicode 6.3.0,
    /// no newly encoded numeric characters will be given `Numeric_Type=Digit`,
    /// nor will existing characters with `Numeric_Type=Numeric` be changed to `Numeric_Type=Digit`.
    /// The distinction between those two types is not considered useful.
    pub digit_numeric_value: Option<u8>,

    /// (8) If the character has the property value `Numeric_Type=Numeric`,
    /// then the `Numeric_Value` of that character is represented with a positive or negative
    /// integer or rational number in this field, and fields 6 and 7 are null.
    /// This includes fractions such as, for example, "1/5" for U+2155 VULGAR FRACTION ONE FIFTH.
    ///
    /// Some characters have these properties based on values from the Unihan data files.
    /// See [`Numeric_Type`, Han].
    ///
    /// [`Numeric_Type`, Han]: http://unicode.org/reports/tr44/#Numeric_Type_Han
    pub numeric_numeric_value: Option<String>,

    /// (9) If the character is a "mirrored" character in bidirectional text,
    /// this field has the value "Y" [true]; otherwise "N" [false].
    /// See *Section 4.7, Bidi Mirrored* of *[Unicode]*.
    /// *Do not confuse this with the [`Bidi_Mirroring_Glyph`] property*.
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    /// [`Bidi_Mirroring_Glyph`]: http://unicode.org/reports/tr44/#Bidi_Mirroring_Glyph
    pub bidi_mirrored: bool,

    /// (10) Old name as published in Unicode 1.0 or ISO 6429 names for control functions.
    /// This field is empty unless it is significantly different from the current name
    /// for the character. No longer used in code chart production. See [`Name_Alias`].
    ///
    /// [`Name_Alias`]: http://unicode.org/reports/tr44/#Name_Alias
    pub unicode_1_name: Option<String>,

    /// (11) ISO 10646 comment field.
    /// It was used for notes that appeared in parentheses in the 10646 names list,
    /// or contained an asterisk to mark an Annex P note.
    ///
    /// As of Unicode 5.2.0, this field no longer contains any non-null values.
    pub iso_comment: (),

    /// (12) Simple uppercase mapping (single character result).
    /// If a character is part of an alphabet with case distinctions,
    /// and has a simple uppercase equivalent, then the uppercase equivalent is in this field.
    /// The simple mappings have a single character result,
    /// where the full mappings may have multi-character results.
    /// For more information, see [Case and Case Mapping].
    ///
    /// [Case and Case Mapping]: http://unicode.org/reports/tr44/#Casemapping
    pub simple_uppercase_mapping: Option<char>,

    /// (13) Simple lowercase mapping (single character result).
    pub simple_lowercase_mapping: Option<char>,

    /// (14) Simple titlecase mapping (single character result).
    pub simple_titlecase_mapping: Option<char>,
    // Implementation note:
    // This struct is currently 200 bytes wide.
    // If the Option<char> are changed to char, it is only 184 bytes.
    // At 0x110000 elements, that's the difference between 196 and 179 MiB.
    // That's a 17 MiB difference.
    //
    // We can get that back pretty much "for free" by storing a u32
    // instead of a Option<char> and treating u32::MAX as None.
    //
    // That's what the [Optional](https://crates.io/crates/optional) crate does.
    //
    // The two u8 -> Option<u8> don't cost extra space, likely due to alignment.
    // For consistency, they should probably also be changed since their valid range
    // is only 0..9.
    //
    // It would also be trivial to provide this functionality locally.
    // That would also give the benefit of allowing us to make a Noned-char,
    // by unsafe-hacking a u32::MAX char, and not losing any application space.
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnicodeData {
    pub entries: Box<[UnicodeDataEntry]>,
}

impl FromStr for UnicodeData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^
                  ([[:xdigit:]]{4,6});                           # codepoint
                  ([^;]+);                                       # name
                  ([^;]+);                                       # general_category
                  ([[:digit:]]+);                                # canonical_combining_class
                  ([^;]+);                                       # bidi_class
                  (?:
                    (?:<([^>]*)>\x20)?                           # decomposition_type (option)
                    ((?:[[:xdigit:]]{4,6}\x20)*[[:xdigit:]]{4,6})# decomposition_mapping (option)
                  )?;
                  ([[:digit:]])?;                                # decimal_numeric_type (option)
                  ([[:digit:]])?;                                # digit_numeric_type (option)
                  ([^;]+)?;                                      # numeric_numeric_type (option)
                  ([YN]);                                        # bidi_mirrored
                  ([^;]+)?;                                      # unicode_1_name (option)
                  ;                                              # iso_comment (void)
                  ([[:xdigit:]]{4,6})?;                          # simple_uppercase_mapping (option)
                  ([[:xdigit:]]{4,6})?;                          # simple_lowercase_mapping (option)
                  ([[:xdigit:]]{4,6})?                           # simple_titlecase_mapping (option)
                $",
            )
            .unwrap();
        }

        // ## Implementation Note
        //
        // 0x4_4000 (278_528) is one quarter of the possible Unicode values (0x11_0000).
        //
        // As of Unicode 10, 274_235 (0x4_2F3B) code points are assigned (excluding surrogates).
        //
        // @CAD97 picked one quarter of the full range because doubling capacity as needed
        // will never go out of the required range and this is a much smaller required allocation
        // for the near future -- for the next 4000 characters assigned.
        let mut entries = Vec::with_capacity(0x4_4000);
        let mut start = None;

        for capture in REGEX.captures_iter(str) {
            if let Some(character) = char::from_u32(u32::from_str_radix(&capture[1], 16).unwrap()) {
                let mut entry = UnicodeDataEntry {
                    character,
                    name: capture[2].to_owned(),
                    general_category: capture[3].to_owned(),
                    canonical_combining_class: capture[4].parse().unwrap(),
                    bidi_class: capture[5].to_owned(),
                    decomposition_type: capture.get(6).map(|m| m.as_str().to_owned()),
                    decomposition_mapping: capture.get(7).map(|m| {
                        m.as_str()
                            .split(' ')
                            .map(|s| u32::from_str_radix(s, 16).unwrap())
                            .map(|codepoint| char::from_u32(codepoint).unwrap())
                            .collect::<Vec<_>>()
                            .into_boxed_slice()
                    }),
                    decimal_numeric_value: capture.get(8).map(|m| m.as_str().parse().unwrap()),
                    digit_numeric_value: capture.get(9).map(|m| m.as_str().parse().unwrap()),
                    numeric_numeric_value: capture.get(10).map(|m| m.as_str().to_owned()),
                    bidi_mirrored: &capture[11] == "Y",
                    unicode_1_name: capture.get(12).map(|m| m.as_str().to_owned()),
                    iso_comment: (),
                    simple_uppercase_mapping: capture
                        .get(13)
                        .map(|m| u32::from_str_radix(m.as_str(), 16).unwrap())
                        .map(|codepoint| char::from_u32(codepoint).unwrap()),
                    simple_lowercase_mapping: capture
                        .get(14)
                        .map(|m| u32::from_str_radix(m.as_str(), 16).unwrap())
                        .map(|codepoint| char::from_u32(codepoint).unwrap()),
                    simple_titlecase_mapping: capture
                        .get(15)
                        .map(|m| u32::from_str_radix(m.as_str(), 16).unwrap())
                        .map(|codepoint| char::from_u32(codepoint).unwrap()),
                };

                entry.simple_titlecase_mapping = entry
                    .simple_titlecase_mapping
                    .or(entry.simple_uppercase_mapping);

                if entry.name.ends_with(", First>") {
                    start = Some(entry.character);
                    let comma_idx = entry.name.find(',').unwrap();
                    let angle_idx = entry.name.find('>').unwrap();
                    entry.name.drain(comma_idx..angle_idx);
                } else if entry.name.ends_with(", Last>") {
                    let start = start.expect("Missing range start");
                    let end = entry.character;
                    assert!(start < end);
                    let comma_idx = entry.name.find(',').unwrap();
                    let angle_idx = entry.name.find('>').unwrap();
                    entry.name.drain(comma_idx..angle_idx);
                    for codepoint in (start as u32)..(end as u32) {
                        if let Some(character) = char::from_u32(codepoint) {
                            entry.character = character;
                            entries.push(entry.clone());
                        }
                    }
                    entry.character = end;
                }
                entries.push(entry);
            }
        }

        Ok(UnicodeData {
            entries: entries.into_boxed_slice(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{UnicodeData, UnicodeDataEntry};

    #[test]
    /// These are 5 randomly selected test cases (sorted for convenience)
    fn unicode_data_entry_parse() {
        assert_eq!(
            "1F35;GREEK SMALL LETTER IOTA WITH DASIA AND OXIA;Ll;0;L;1F31 0301;;;;N;;;1F3D;;1F3D\n\
             A86D;PHAGS-PA LETTER ALTERNATE YA;Lo;0;L;;;;;N;;;;;\n\
             2D01;GEORGIAN SMALL LETTER BAN;Ll;0;L;;;;;N;;;10A1;;10A1\n\
             13060;EGYPTIAN HIEROGLYPH C004;Lo;0;L;;;;;N;;;;;\n\
             1B042;HENTAIGANA LETTER SA-7;Lo;0;L;;;;;N;;;;;\n\
             "
            .parse(),
            Ok(UnicodeData {
                entries: vec![
                    UnicodeDataEntry {
                        character: '\u{1F35}',
                        name: "GREEK SMALL LETTER IOTA WITH DASIA AND OXIA".to_owned(),
                        general_category: "Ll".to_owned(),
                        canonical_combining_class: 0,
                        bidi_class: "L".to_owned(),
                        decomposition_type: None,
                        decomposition_mapping: Some(
                            vec!['\u{1F31}', '\u{0301}'].into_boxed_slice(),
                        ),
                        decimal_numeric_value: None,
                        digit_numeric_value: None,
                        numeric_numeric_value: None,
                        bidi_mirrored: false,
                        unicode_1_name: None,
                        iso_comment: (),
                        simple_uppercase_mapping: Some('\u{1F3D}'),
                        simple_lowercase_mapping: None,
                        simple_titlecase_mapping: Some('\u{1F3D}'),
                    },
                    UnicodeDataEntry {
                        character: '\u{A86D}',
                        name: "PHAGS-PA LETTER ALTERNATE YA".to_owned(),
                        general_category: "Lo".to_owned(),
                        canonical_combining_class: 0,
                        bidi_class: "L".to_owned(),
                        decomposition_type: None,
                        decomposition_mapping: None,
                        decimal_numeric_value: None,
                        digit_numeric_value: None,
                        numeric_numeric_value: None,
                        bidi_mirrored: false,
                        unicode_1_name: None,
                        iso_comment: (),
                        simple_uppercase_mapping: None,
                        simple_lowercase_mapping: None,
                        simple_titlecase_mapping: None,
                    },
                    UnicodeDataEntry {
                        character: '\u{2D01}',
                        name: "GEORGIAN SMALL LETTER BAN".to_owned(),
                        general_category: "Ll".to_owned(),
                        canonical_combining_class: 0,
                        bidi_class: "L".to_owned(),
                        decomposition_type: None,
                        decomposition_mapping: None,
                        decimal_numeric_value: None,
                        digit_numeric_value: None,
                        numeric_numeric_value: None,
                        bidi_mirrored: false,
                        unicode_1_name: None,
                        iso_comment: (),
                        simple_uppercase_mapping: Some('\u{10A1}'),
                        simple_lowercase_mapping: None,
                        simple_titlecase_mapping: Some('\u{10A1}'),
                    },
                    UnicodeDataEntry {
                        character: '\u{13060}',
                        name: "EGYPTIAN HIEROGLYPH C004".to_owned(),
                        general_category: "Lo".to_owned(),
                        canonical_combining_class: 0,
                        bidi_class: "L".to_owned(),
                        decomposition_type: None,
                        decomposition_mapping: None,
                        decimal_numeric_value: None,
                        digit_numeric_value: None,
                        numeric_numeric_value: None,
                        bidi_mirrored: false,
                        unicode_1_name: None,
                        iso_comment: (),
                        simple_uppercase_mapping: None,
                        simple_lowercase_mapping: None,
                        simple_titlecase_mapping: None,
                    },
                    UnicodeDataEntry {
                        character: '\u{1B042}',
                        name: "HENTAIGANA LETTER SA-7".to_owned(),
                        general_category: "Lo".to_owned(),
                        canonical_combining_class: 0,
                        bidi_class: "L".to_owned(),
                        decomposition_type: None,
                        decomposition_mapping: None,
                        decimal_numeric_value: None,
                        digit_numeric_value: None,
                        numeric_numeric_value: None,
                        bidi_mirrored: false,
                        unicode_1_name: None,
                        iso_comment: (),
                        simple_uppercase_mapping: None,
                        simple_lowercase_mapping: None,
                        simple_titlecase_mapping: None,
                    },
                ]
                .into_boxed_slice(),
            })
        );
    }
}
