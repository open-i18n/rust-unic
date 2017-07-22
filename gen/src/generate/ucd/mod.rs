// Remove the too-long line messages
#![cfg_attr(rustfmt, rustfmt_skip)]

// FIXME: This file got really big for a mod.rs

pub mod core;
pub mod age;

use std::char;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::str::FromStr;

use generate::PREAMBLE;

use regex::Regex;

lazy_static! {
    /// Version of the local UCD files
    static ref UNICODE_VERSION: (u16, u16, u16) = {
        let mut readme = File::open(Path::new("data/ucd/ReadMe.txt"))
            .expect("Failed to open UCD ReadMe. Did you run -u?");
        let mut buffer = String::new();
        readme.read_to_string(&mut buffer)
            .expect("Failed to read UCD ReadMe. Did you run -u?");
        // This is a valid regex and should therefor never panic.
        let pattern = Regex::new(r"for Version (\d+).(\d+).(\d+)").unwrap();
        let captures = pattern.captures(&buffer)
            .expect("Regex didn't match UCD ReadMe. Did it download correctly?");
        (
            // These unwrap should never panic due to the regex.
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
            captures.get(3).unwrap().as_str().parse().unwrap(),
        )
    };
}

impl UNICODE_VERSION {
    /// Emit `unicode_version.rsv` into a directory.
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("unicode_version.rsv"))?;
        write!(file, "{}", PREAMBLE)?;
        writeln!(
            file,
            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
            self.0,
            self.1,
            self.2,
        )?;
        Ok(())
    }
}

/// Data line from UnicodeData.txt
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnicodeDataEntry {
    pub codepoint: u32,

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
    /// For the property values, see [General Category Values].
    ///
    /// [General Category Values]: http://unicode.org/reports/tr44/#General_Category_Values
    pub general_category: String,

    /// (3) The classes used for the Canonical Ordering Algorithm in the Unicode Standard.
    /// This property could be considered either an enumerated property or a numeric property:
    /// the principal use of the property is in terms of the numeric values.
    /// For the property value names associated with different numeric values,
    /// see [DerivedCombiningClass.txt] and [Canonical Combining Class Values].
    ///
    /// [DerivedCombiningClass.txt]: http://unicode.org/reports/tr44/#DerivedCombiningClass.txt
    /// [Canonical Combining Class Values]: http://unicode.org/reports/tr44/#Canonical_Combining_Class_Values
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

    /// (5) This field contains both values [`Decomposition_Type` and `Decomposition_Mapping`],
    /// with the type in angle brackets.
    /// The decomposition mappings exactly match the decomposition mappings
    /// published with the character names in the Unicode Standard.
    /// For more information, see [Character Decomposition Mappings].
    ///
    /// [Character Decomposition Mappings]: http://unicode.org/reports/tr44/#Character_Decomposition_Mappings
    pub decomposition: Option<String>,

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
    // This struct is currently 184 bytes wide.
    // If the Option<char> are changed to char, it is only 168 bytes.
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

lazy_static! {
    static ref UNICODE_DATA_ENTRY_REGEX: Regex = Regex::new("\
        ^([[:xdigit:]]{4,6});\
         ([^;]+);([^;]+);\
         ([[:digit:]]+);\
         ([^;]+);([^;]*);\
         ([[:digit:]]?);\
         ([[:digit:]]?);\
         ([^;]*);([YN]);\
         ([^;]*);;\
         ([[:xdigit:]]*);\
         ([[:xdigit:]]*);\
         ([[:xdigit:]]*)$\
    ").unwrap();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl FromStr for UnicodeDataEntry {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        UNICODE_DATA_ENTRY_REGEX
            .captures(str)
            .map(|matches| {
                UnicodeDataEntry {
                    codepoint: u32::from_str_radix(matches.get(1).unwrap().as_str(), 16).unwrap(),
                    name: matches.get(2).unwrap().as_str().to_owned(),
                    general_category: matches.get(3).unwrap().as_str().to_owned(),
                    canonical_combining_class: matches.get(4).unwrap().as_str().parse().unwrap(),
                    bidi_class: matches.get(5).unwrap().as_str().to_owned(),
                    decomposition: {
                        let str = matches.get(6).unwrap().as_str();
                        if str.is_empty() {
                            None
                        } else {
                            Some(str.to_owned())
                        }
                    },
                    decimal_numeric_value: {
                        let str = matches.get(7).unwrap().as_str();
                        if str.is_empty() {
                            None
                        } else {
                            Some(str.parse().unwrap())
                        }
                    },
                    digit_numeric_value: {
                        let str = matches.get(8).unwrap().as_str();
                        if str.is_empty() {
                            None
                        } else {
                            Some(str.parse().unwrap())
                        }
                    },
                    numeric_numeric_value: {
                        let str = matches.get(9).unwrap().as_str();
                        if str.is_empty() {
                            None
                        } else {
                            Some(str.to_owned())
                        }
                    },
                    bidi_mirrored: matches.get(10).unwrap().as_str() == "Y",
                    unicode_1_name: {
                        let str = matches.get(11).unwrap().as_str();
                        if str.is_empty() {
                            None
                        } else {
                            Some(str.to_owned())
                        }
                    },
                    // `Err` value: Syntax("Empty regex groups (e.g., '()') are not allowed.")
                    iso_comment: (),
                    simple_uppercase_mapping:
                        u32::from_str_radix(matches.get(12).unwrap().as_str(), 16)
                            .ok().and_then(char::from_u32),
                    simple_lowercase_mapping:
                        u32::from_str_radix(matches.get(13).unwrap().as_str(), 16)
                            .ok().and_then(char::from_u32),
                    simple_titlecase_mapping:
                        u32::from_str_radix(matches.get(14).unwrap().as_str(), 16)
                            .ok().and_then(char::from_u32)
                            .or_else(|| {
                                // defaults to simple_uppercase_mapping
                                u32::from_str_radix(matches.get(12).unwrap().as_str(), 16)
                                    .ok().and_then(char::from_u32)
                            }),
                }
            })
            .ok_or(())
    }
}

/// Manual impl to skip fields which (should) be the same
impl PartialOrd for UnicodeDataEntry {
    fn partial_cmp(&self, other: &UnicodeDataEntry) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.codepoint, &other.codepoint)
    }
}

/// Manual impl to skip fields which (should) be the same
impl Ord for UnicodeDataEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.codepoint, &other.codepoint)
    }
}

lazy_static! {
    pub static ref UNICODE_DATA: Box<[UnicodeDataEntry]> = {
        /// impl note:
        /// 0x44000 (278528) is one quarter of the possible Unicode values (0x110000).
        /// As of Unicode 10, 274235 (0x42F3B) code points are assigned (excluding surrogates).
        /// I (@CAD97) picked one quarter of the full range because doubling capacity as needed
        /// will never go out of the required range and this is a much smaller required allocation
        /// for the near future for the next 4000 characters assigned.
        let mut unicode_data = Vec::with_capacity(0x44000);
        let mut range_start: Option<u32> = None;
        let file = File::open(Path::new("data/ucd/UnicodeData.txt"))
            .expect("Failed to open UCD UnicodeData. Did you run -u?");

        for line in BufReader::new(file).lines() {
            let line = line.expect("Invalid string in UnicodeData. Did it get corrupted?");
            let entry = UnicodeDataEntry::from_str(&line)
                .expect("Error parsing UnicodeData");

            if char::from_u32(entry.codepoint).is_none() {
                continue;
            }

            if entry.name.ends_with(", First>") {
                range_start = Some(entry.codepoint);
                let mut data = entry;
                let comma_idx = data.name.find(',').unwrap();
                let angle_idx = data.name.find('>').unwrap();
                data.name.drain(comma_idx..angle_idx);
                unicode_data.push(data);
            } else if let Some(start) = range_start {
                assert!(entry.name.ends_with(", Last>"));
                range_start = None;
                let end = entry.codepoint;
                let mut data = entry;
                let comma_idx = data.name.find(',').unwrap();
                let angle_idx = data.name.find('>').unwrap();
                data.name.drain(comma_idx..angle_idx);
                for i in start..end {
                    data.codepoint = i;
                    unicode_data.push(data.clone());
                }
                data.codepoint = end;
                unicode_data.push(data);
            } else {
                unicode_data.push(entry);
            }
        }

        unicode_data.into_boxed_slice()
    };
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::UnicodeDataEntry;

    #[test]
    /// These are 5 randomly selected test cases (sorted for convenience)
    fn unicode_data_entry_parse() {
        assert_eq!(
            UnicodeDataEntry::from_str(
                "1F35;GREEK SMALL LETTER IOTA WITH DASIA AND OXIA;Ll;0;L;1F31 0301;;;;N;;;1F3D;;1F3D"
            ),
            Ok(UnicodeDataEntry {
                codepoint: 0x1F35,
                name: "GREEK SMALL LETTER IOTA WITH DASIA AND OXIA".to_owned(),
                general_category: "Ll".to_owned(),
                canonical_combining_class: 0,
                bidi_class: "L".to_owned(),
                decomposition: Some("1F31 0301".to_owned()),
                decimal_numeric_value: None,
                digit_numeric_value: None,
                numeric_numeric_value: None,
                bidi_mirrored: false,
                unicode_1_name: None,
                iso_comment: (),
                simple_uppercase_mapping: Some('\u{1F3D}'),
                simple_lowercase_mapping: None,
                simple_titlecase_mapping: Some('\u{1F3D}'),
            })
        );
        assert_eq!(
            UnicodeDataEntry::from_str("A86D;PHAGS-PA LETTER ALTERNATE YA;Lo;0;L;;;;;N;;;;;"),
            Ok(UnicodeDataEntry {
                codepoint: 0xA86D,
                name: "PHAGS-PA LETTER ALTERNATE YA".to_owned(),
                general_category: "Lo".to_owned(),
                canonical_combining_class: 0,
                bidi_class: "L".to_owned(),
                decomposition: None,
                decimal_numeric_value: None,
                digit_numeric_value: None,
                numeric_numeric_value: None,
                bidi_mirrored: false,
                unicode_1_name: None,
                iso_comment: (),
                simple_uppercase_mapping: None,
                simple_lowercase_mapping: None,
                simple_titlecase_mapping: None,
            })
        );
        assert_eq!(
            UnicodeDataEntry::from_str("2D01;GEORGIAN SMALL LETTER BAN;Ll;0;L;;;;;N;;;10A1;;10A1"),
            Ok(UnicodeDataEntry {
                codepoint: 0x2D01,
                name: "GEORGIAN SMALL LETTER BAN".to_owned(),
                general_category: "Ll".to_owned(),
                canonical_combining_class: 0,
                bidi_class: "L".to_owned(),
                decomposition: None,
                decimal_numeric_value: None,
                digit_numeric_value: None,
                numeric_numeric_value: None,
                bidi_mirrored: false,
                unicode_1_name: None,
                iso_comment: (),
                simple_uppercase_mapping: Some('\u{10A1}'),
                simple_lowercase_mapping: None,
                simple_titlecase_mapping: Some('\u{10A1}'),
            })
        );
        assert_eq!(
            UnicodeDataEntry::from_str("13060;EGYPTIAN HIEROGLYPH C004;Lo;0;L;;;;;N;;;;;"),
            Ok(UnicodeDataEntry {
                codepoint: 0x13060,
                name: "EGYPTIAN HIEROGLYPH C004".to_owned(),
                general_category: "Lo".to_owned(),
                canonical_combining_class: 0,
                bidi_class: "L".to_owned(),
                decomposition: None,
                decimal_numeric_value: None,
                digit_numeric_value: None,
                numeric_numeric_value: None,
                bidi_mirrored: false,
                unicode_1_name: None,
                iso_comment: (),
                simple_uppercase_mapping: None,
                simple_lowercase_mapping: None,
                simple_titlecase_mapping: None,
            })
        );
        assert_eq!(
            UnicodeDataEntry::from_str("1B042;HENTAIGANA LETTER SA-7;Lo;0;L;;;;;N;;;;;"),
            Ok(UnicodeDataEntry {
                codepoint: 0x1B042,
                name: "HENTAIGANA LETTER SA-7".to_owned(),
                general_category: "Lo".to_owned(),
                canonical_combining_class: 0,
                bidi_class: "L".to_owned(),
                decomposition: None,
                decimal_numeric_value: None,
                digit_numeric_value: None,
                numeric_numeric_value: None,
                bidi_mirrored: false,
                unicode_1_name: None,
                iso_comment: (),
                simple_uppercase_mapping: None,
                simple_lowercase_mapping: None,
                simple_titlecase_mapping: None,
            })
        );
    }
}
