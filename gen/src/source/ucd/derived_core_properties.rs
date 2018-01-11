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

use source::utils::read;

use super::BINARY_PROPERTIES_REGEX;

lazy_static! {
    pub static ref DERIVED_CORE_PROPERTIES: DerivedCoreProperties = {
        read("data/ucd/DerivedCoreProperties.txt").parse().unwrap()
    };
}

#[derive(Clone, Debug, Default)]
pub struct DerivedCoreProperties {
    /// Characters with the `Lowercase` property. For more information,
    /// see _Chapter 4, Character Properties_ in ***[Unicode]***.
    ///
    /// _Equivalent to `gc=Ll + Other_Lowercase`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub lowercase: BTreeSet<char>,

    /// Characters with the `Uppercase` property. For more information,
    /// see _Chapter 4, Character Properties_ in ***[Unicode]***.
    ///
    /// _Equivalent to `gc=Lu + Other_Uppercase`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub uppercase: BTreeSet<char>,

    /// Characters wich are considered to be either uppercase, lowercase, or titlecase characters.
    /// This property is not identical to the `Changes_When_Casemapped_ property.
    /// For more information, see D125 in _§3.13 Default Case Algorithms_ in ***[Unicode]***.
    ///
    /// _Equivalent to `Lowercase + Uppercase + gc=Lt`
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub cased: BTreeSet<char>,

    /// Characters which are ignored for casing purposes.
    /// For more information, see D125 in _§3.13 Default Case Algorithms_ in ***[Unicode]***.
    ///
    /// _Equivalent to `gc=Mn|Me|Cf|Lm|Sk + Word_Break=MidLetter|MidNumLet|Single_Quote`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub case_ignorable: BTreeSet<char>,

    /// Characters whose normalized forms are not stable under a `toLowercase` mapping.
    /// For more information, see D125 in _§3.13 Default Case Algorithms_ in ***[Unicode]***.
    ///
    /// _Equivalent to `toLowercase(toNFD(X)) != toNFD(X)`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub changes_when_lowercased: BTreeSet<char>,

    /// Characters whose normalized forms are not stable under a `toUppercase` mapping.
    /// For more information, see D125 in _§3.13 Default Case Algorithms_ in ***[Unicode]***.
    ///
    /// _Equivalent to `toUppercase(toNFD(X)) != toNFD(X)`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub changes_when_uppercased: BTreeSet<char>,

    /// Characters whose normalized forms are not stable under a `toTitlecase` mapping.
    /// For more information, see D125 in _§3.13 Default Case Algorithms_ in ***[Unicode]***.
    ///
    /// _Equivalent to `toTitlecase(toNFD(X)) != toNFD(X)`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub changes_when_titlecased: BTreeSet<char>,

    /// Characters whose normalized forms are not stable under case folding.
    /// For more information, see D125 in _§3.13 Default Case Algorithms_ in ***[Unicode]***.
    ///
    /// _Equivalent to `toCasefold(toNFD(X)) != toNFD(X)`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub changes_when_casefolded: BTreeSet<char>,

    /// Characters which may change when they undergo case mapping.
    /// For more information, see D125 in _§3.13 Default Case Algorithms_ in ***[Unicode]***.
    ///
    /// _Equivalent to
    /// `Changes_When_Lowercased(X) || Changes_When_Uppercased(X) || Changes_When_Titlecased(X)`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub changes_when_casemapped: BTreeSet<char>,

    /// Characters with the `Alphabetic` property. For more information,
    /// see _Chapter 4, Character Properties_ in ***[Unicode]***.
    ///
    /// _Equivalent to `Lowercase + uppercase + gc=Lt|Lm|Lo|Nl + Other_Alphabetic`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub alphabetic: BTreeSet<char>,

    /// For programmatic determination of default ignorable code points.
    /// New characters that should be ignored in rendering (unless explicitly supported)
    /// will be assigned in these ranges, permitting programs to correctly handle
    /// the default rendering of such characters when not otherwise supported.
    /// For more information, see the FAQ [Display of Unsupported Characters],
    /// and _§5.21 Ignoring Characters in Processing_ in ***[Unicode]***.
    ///
    /// _Equivalent to:_
    ///
    ///     Other_Default_Ignorable_Code_Point
    ///     + gc=Cf (format characters)
    ///     + Variation_Selector
    ///     - White_Space
    ///     - U+FFF9-FFFB (annotation characters)
    ///     - U+0600-0605, U+06DD, U+070F, U+08E2, U+110BD (Cf characters that should be visible)
    ///
    /// [Display of Unsupported Characters]: https://www.unicode.org/faq/unsup_char.html
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub default_ignorable_code_point: BTreeSet<char>,

    /// Property used together with the definition of Standard Korean Syllable Block
    /// to define "Grapheme base". See D58 in _Chapter 3, Conformance_ in ***[Unicode]***.
    ///
    /// _Equivalent to `U+0000-10FFFF -gc=Cc|Cf|Cs|Co|Cn|Zl|Zp - Grapheme_Extend`_
    ///
    /// **Note:** `Grapheme_Base` is a property of individual characters.
    /// That usage contrasts with "grapheme base", which is an attribute of Unicode strings;
    /// a grapheme base may consist of a Korean syllable
    /// which is itself represented by a sequence of conjoining jamos.
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub grapheme_base: BTreeSet<char>,

    /// Property used to define "Grapheme extender".
    /// See D59 in _Chapter 3, Conformance_ in ***[Unicode]***.
    ///
    /// _Equivalent to `gc=Me|Mn + Other_Grapheme_Extend`_
    ///
    /// **Note:** The set of characters for which `Grapheme_Extend=Yes`
    /// is equivalent to the set of characters for which `Grapheme_Cluster_Break=Extend`.
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub grapheme_extend: BTreeSet<char>,

    // Grapheme_Link is deprecated
    /// Characters with the `Math` property. For more information,
    /// see _Chapter 4, Character Properties_ in ***[Unicode]***.
    ///
    /// _Equivalent to `gc=Sm + Other_Math`_
    ///
    /// [Unicode]: http://unicode.org/reports/tr41/tr41-21.html#Unicode
    pub math: BTreeSet<char>,

    /// Used to determine programming identifiers, as described in
    /// [Unicode Standard Annex #31, "Unicode Identifier and Pattern Syntax"][UAX31]
    ///
    /// [UAX31]: http://unicode.org/reports/tr41/tr41-21.html#UAX31
    pub id_start: BTreeSet<char>,

    /// Used to determine programming identifiers, as described in
    /// [Unicode Standard Annex #31, "Unicode Identifier and Pattern Syntax"][UAX31]
    ///
    /// [UAX31]: http://unicode.org/reports/tr41/tr41-21.html#UAX31
    pub id_continue: BTreeSet<char>,

    /// Used to determine programming identifiers, as described in
    /// [Unicode Standard Annex #31, "Unicode Identifier and Pattern Syntax"][UAX31]
    ///
    /// [UAX31]: http://unicode.org/reports/tr41/tr41-21.html#UAX31
    pub xid_start: BTreeSet<char>,

    /// Used to determine programming identifiers, as described in
    /// [Unicode Standard Annex #31, "Unicode Identifier and Pattern Syntax"][UAX31]
    ///
    /// [UAX31]: http://unicode.org/reports/tr41/tr41-21.html#UAX31
    pub xid_continue: BTreeSet<char>,
}

impl FromStr for DerivedCoreProperties {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut props = DerivedCoreProperties::default();

        for capture in BINARY_PROPERTIES_REGEX.captures_iter(str) {
            let low = char::from_u32(u32::from_str_radix(&capture[1], 16).unwrap()).unwrap();
            let high = capture
                .get(2)
                .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                .map(|u| char::from_u32(u).unwrap())
                .unwrap_or(low);
            let range = chars!(low..=high);

            match &capture[3] {
                "Lowercase" => props.lowercase.extend(range),
                "Uppercase" => props.uppercase.extend(range),
                "Cased" => props.cased.extend(range),
                "Case_Ignorable" => props.case_ignorable.extend(range),
                "Changes_When_Lowercased" => props.changes_when_lowercased.extend(range),
                "Changes_When_Uppercased" => props.changes_when_uppercased.extend(range),
                "Changes_When_Titlecased" => props.changes_when_titlecased.extend(range),
                "Changes_When_Casefolded" => props.changes_when_casefolded.extend(range),
                "Changes_When_Casemapped" => props.changes_when_casemapped.extend(range),
                "Alphabetic" => props.alphabetic.extend(range),
                "Default_Ignorable_Code_Point" => props.default_ignorable_code_point.extend(range),
                "Grapheme_Extend" => props.grapheme_extend.extend(range),
                "Grapheme_Base" => props.grapheme_base.extend(range),
                "Grapheme_Link" => { /* ignored */ }
                "Math" => props.math.extend(range),
                "ID_Start" => props.id_start.extend(range),
                "ID_Continue" => props.id_continue.extend(range),
                "XID_Start" => props.xid_start.extend(range),
                "XID_Continue" => props.xid_continue.extend(range),
                prop => panic!("Unsupported DerivedCoreProperty `{}`", prop),
            }
        }

        Ok(props)
    }
}
