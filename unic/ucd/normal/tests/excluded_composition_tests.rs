// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_ucd_normal;

use unic_ucd_normal::{canonical_composition, canonical_decomposition};

// Consider: could be conformance pulling from UCD Composition Exclusions
// <https://www.unicode.org/Public/10.0.0/ucd/CompositionExclusions.txt>
// (Relevant tables generated with DerivedNormalizationProps)
// As is, the battery is manually extracted from that file.
#[test]
/// Require that excluded composition characters are not suggested in the composition tables
fn test_composition_exclusions() {
    // (1) Script Specifics
    let battery = &[
        '\u{0958}', //  DEVANAGARI LETTER QA
        '\u{0959}', //  DEVANAGARI LETTER KHHA
        '\u{095A}', //  DEVANAGARI LETTER GHHA
        '\u{095B}', //  DEVANAGARI LETTER ZA
        '\u{095C}', //  DEVANAGARI LETTER DDDHA
        '\u{095D}', //  DEVANAGARI LETTER RHA
        '\u{095E}', //  DEVANAGARI LETTER FA
        '\u{095F}', //  DEVANAGARI LETTER YYA
        '\u{09DC}', //  BENGALI LETTER RRA
        '\u{09DD}', //  BENGALI LETTER RHA
        '\u{09DF}', //  BENGALI LETTER YYA
        '\u{0A33}', //  GURMUKHI LETTER LLA
        '\u{0A36}', //  GURMUKHI LETTER SHA
        '\u{0A59}', //  GURMUKHI LETTER KHHA
        '\u{0A5A}', //  GURMUKHI LETTER GHHA
        '\u{0A5B}', //  GURMUKHI LETTER ZA
        '\u{0A5E}', //  GURMUKHI LETTER FA
        '\u{0B5C}', //  ORIYA LETTER RRA
        '\u{0B5D}', //  ORIYA LETTER RHA
        '\u{0F43}', //  TIBETAN LETTER GHA
        '\u{0F4D}', //  TIBETAN LETTER DDHA
        '\u{0F52}', //  TIBETAN LETTER DHA
        '\u{0F57}', //  TIBETAN LETTER BHA
        '\u{0F5C}', //  TIBETAN LETTER DZHA
        '\u{0F69}', //  TIBETAN LETTER KSSA
        '\u{0F76}', //  TIBETAN VOWEL SIGN VOCALIC R
        '\u{0F78}', //  TIBETAN VOWEL SIGN VOCALIC L
        '\u{0F93}', //  TIBETAN SUBJOINED LETTER GHA
        '\u{0F9D}', //  TIBETAN SUBJOINED LETTER DDHA
        '\u{0FA2}', //  TIBETAN SUBJOINED LETTER DHA
        '\u{0FA7}', //  TIBETAN SUBJOINED LETTER BHA
        '\u{0FAC}', //  TIBETAN SUBJOINED LETTER DZHA
        '\u{0FB9}', //  TIBETAN SUBJOINED LETTER KSSA
        '\u{FB1D}', //  HEBREW LETTER YOD WITH HIRIQ
        '\u{FB1F}', //  HEBREW LIGATURE YIDDISH YOD YOD PATAH
        '\u{FB2A}', //  HEBREW LETTER SHIN WITH SHIN DOT
        '\u{FB2B}', //  HEBREW LETTER SHIN WITH SIN DOT
        '\u{FB2C}', //  HEBREW LETTER SHIN WITH DAGESH AND SHIN DOT
        '\u{FB2D}', //  HEBREW LETTER SHIN WITH DAGESH AND SIN DOT
        '\u{FB2E}', //  HEBREW LETTER ALEF WITH PATAH
        '\u{FB2F}', //  HEBREW LETTER ALEF WITH QAMATS
        '\u{FB30}', //  HEBREW LETTER ALEF WITH MAPIQ
        '\u{FB31}', //  HEBREW LETTER BET WITH DAGESH
        '\u{FB32}', //  HEBREW LETTER GIMEL WITH DAGESH
        '\u{FB33}', //  HEBREW LETTER DALET WITH DAGESH
        '\u{FB34}', //  HEBREW LETTER HE WITH MAPIQ
        '\u{FB35}', //  HEBREW LETTER VAV WITH DAGESH
        '\u{FB36}', //  HEBREW LETTER ZAYIN WITH DAGESH
        '\u{FB38}', //  HEBREW LETTER TET WITH DAGESH
        '\u{FB39}', //  HEBREW LETTER YOD WITH DAGESH
        '\u{FB3A}', //  HEBREW LETTER FINAL KAF WITH DAGESH
        '\u{FB3B}', //  HEBREW LETTER KAF WITH DAGESH
        '\u{FB3C}', //  HEBREW LETTER LAMED WITH DAGESH
        '\u{FB3E}', //  HEBREW LETTER MEM WITH DAGESH
        '\u{FB40}', //  HEBREW LETTER NUN WITH DAGESH
        '\u{FB41}', //  HEBREW LETTER SAMEKH WITH DAGESH
        '\u{FB43}', //  HEBREW LETTER FINAL PE WITH DAGESH
        '\u{FB44}', //  HEBREW LETTER PE WITH DAGESH
        '\u{FB46}', //  HEBREW LETTER TSADI WITH DAGESH
        '\u{FB47}', //  HEBREW LETTER QOF WITH DAGESH
        '\u{FB48}', //  HEBREW LETTER RESH WITH DAGESH
        '\u{FB49}', //  HEBREW LETTER SHIN WITH DAGESH
        '\u{FB4A}', //  HEBREW LETTER TAV WITH DAGESH
        '\u{FB4B}', //  HEBREW LETTER VAV WITH HOLAM
        '\u{FB4C}', //  HEBREW LETTER BET WITH RAFE
        '\u{FB4D}', //  HEBREW LETTER KAF WITH RAFE
        '\u{FB4E}', //  HEBREW LETTER PE WITH RAFE
    ];

    for char in battery.iter() {
        let decomposition = canonical_decomposition(*char).unwrap();
        assert!(!canonical_composition(decomposition[0])
            .unwrap_or_else(Default::default)
            .iter()
            .any(|(follow, _)| follow.low == decomposition[1]));
        assert!(!canonical_composition(decomposition[0])
            .unwrap_or_else(Default::default)
            .iter()
            .any(|(_, ref result)| battery.contains(result)));
    }
}
