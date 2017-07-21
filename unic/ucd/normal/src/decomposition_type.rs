// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Accessor for Decomposition_Type (dt) property

use std::cmp::Ordering;

use composition::canonical_decomposition;


/// Represents the Unicode character
/// [*Decomposition_Type*](http://www.unicode.org/reports/tr44/#Decomposition_Type) property.
///
/// * <http://www.unicode.org/reports/tr44/#Character_Decomposition_Mappings>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DecompositionType {
    Canonical, // abbreviated: Can
    Compat,    // abbreviated: Com
    Circle,    // abbreviated: Enc
    Final,     // abbreviated: Fin
    Font,      // abbreviated: Font
    Fraction,  // abbreviated: Fra
    Initial,   // abbreviated: Init
    Isolated,  // abbreviated: Iso
    Medial,    // abbreviated: Med
    Narrow,    // abbreviated: Nar
    Nobreak,   // abbreviated: Nb
    None,      // abbreviated: None
    Small,     // abbreviated: Sml
    Square,    // abbreviated: Sqr
    Sub,       // abbreviated: Sub
    Super,     // abbreviated: Sup
    Vertical,  // abbreviated: Vert
    Wide,      // abbreviated: Wide
}


use self::DecompositionType::*;

// TODO: Maybe merge this table with compatibility_decomposition_mapping ones
const COMPATIBILITY_DECOMPOSITION_TYPE_TABLE: &'static [(char, char, DecompositionType)] =
    include!("tables/compatibility_decomposition_type_values.rsv");


impl DecompositionType {
    /// Find the DecompositionType of a single char.
    pub fn of(ch: char) -> Option<DecompositionType> {
        // First check the canonical decompositions
        if let Some(_) = canonical_decomposition(ch) {
            return Some(Canonical);
        }
        bsearch_range_value_table(ch, COMPATIBILITY_DECOMPOSITION_TYPE_TABLE)
    }
}

fn bsearch_range_value_table(
    c: char,
    r: &'static [(char, char, DecompositionType)],
) -> Option<DecompositionType> {
    match r.binary_search_by(|&(lo, hi, _)| if lo <= c && c <= hi {
        Ordering::Equal
    } else if hi < c {
        Ordering::Less
    } else {
        Ordering::Greater
    }) {
        Ok(idx) => {
            let (_, _, dt) = r[idx];
            Some(dt)
        }
        Err(_) => Option::None,
    }
}


#[cfg(test)]
mod tests {
    use super::DecompositionType as DT;

    #[test]
    fn test_ascii() {
        assert_eq!(DT::of('\u{0000}'), None);
        assert_eq!(DT::of('\u{0040}'), None);
        assert_eq!(DT::of('\u{0041}'), None);
        assert_eq!(DT::of('\u{0062}'), None);
        assert_eq!(DT::of('\u{007F}'), None);
    }

    #[test]
    fn test_bmp() {
        // Compatibility
        assert_eq!(DT::of('\u{a0}'), Some(DT::Nobreak));
        assert_eq!(DT::of('\u{a8}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{aa}'), Some(DT::Super));
        assert_eq!(DT::of('\u{af}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b2}'), Some(DT::Super));
        assert_eq!(DT::of('\u{b3}'), Some(DT::Super));
        assert_eq!(DT::of('\u{b4}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b5}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b8}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b9}'), Some(DT::Super));
        assert_eq!(DT::of('\u{ba}'), Some(DT::Super));
        assert_eq!(DT::of('\u{bc}'), Some(DT::Fraction));
        assert_eq!(DT::of('\u{bd}'), Some(DT::Fraction));
        assert_eq!(DT::of('\u{be}'), Some(DT::Fraction));

        // Canonical
        assert_eq!(DT::of('\u{c0}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{c1}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{d6}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{d9}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{10f}'), Some(DT::Canonical));

        // Combining Diacritical Marks
        assert_eq!(DT::of('\u{0300}'), None);
        assert_eq!(DT::of('\u{0314}'), None);
        assert_eq!(DT::of('\u{0315}'), None);
        assert_eq!(DT::of('\u{0316}'), None);
        assert_eq!(DT::of('\u{0319}'), None);

        // Hebrew
        assert_eq!(DT::of('\u{0590}'), None);
        assert_eq!(DT::of('\u{05D0}'), None);
        assert_eq!(DT::of('\u{05D1}'), None);
        assert_eq!(DT::of('\u{05FF}'), None);

        // Arabic
        assert_eq!(DT::of('\u{0600}'), None);
        assert_eq!(DT::of('\u{0627}'), None);
        assert_eq!(DT::of('\u{064B}'), None);
        assert_eq!(DT::of('\u{064C}'), None);
        assert_eq!(DT::of('\u{064D}'), None);
        assert_eq!(DT::of('\u{064E}'), None);
        assert_eq!(DT::of('\u{064F}'), None);
        assert_eq!(DT::of('\u{0650}'), None);
        assert_eq!(DT::of('\u{0651}'), None);
        assert_eq!(DT::of('\u{0652}'), None);

        assert_eq!(DT::of('\u{07BF}'), None);
        assert_eq!(DT::of('\u{07C0}'), None);
        assert_eq!(DT::of('\u{085F}'), None);
        assert_eq!(DT::of('\u{0860}'), None);
        assert_eq!(DT::of('\u{0870}'), None);
        assert_eq!(DT::of('\u{089F}'), None);
        assert_eq!(DT::of('\u{08A0}'), None);
        assert_eq!(DT::of('\u{089F}'), None);
        assert_eq!(DT::of('\u{08FF}'), None);

        //  Currency Symbols
        assert_eq!(DT::of('\u{20A0}'), None);
        assert_eq!(DT::of('\u{20CF}'), None);

        // Arabic Presentation Forms
        assert_eq!(DT::of('\u{FB1D}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{FB4F}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{FB50}'), Some(DT::Isolated));
        assert_eq!(DT::of('\u{FDCF}'), None);
        assert_eq!(DT::of('\u{FDF0}'), Some(DT::Isolated));
        assert_eq!(DT::of('\u{FDFF}'), None);
        assert_eq!(DT::of('\u{FE70}'), Some(DT::Isolated));
        assert_eq!(DT::of('\u{FEFE}'), None);
        assert_eq!(DT::of('\u{FEFF}'), None);

        // noncharacters
        assert_eq!(DT::of('\u{FDD0}'), None);
        assert_eq!(DT::of('\u{FDD1}'), None);
        assert_eq!(DT::of('\u{FDEE}'), None);
        assert_eq!(DT::of('\u{FDEF}'), None);
        assert_eq!(DT::of('\u{FFFE}'), None);
        assert_eq!(DT::of('\u{FFFF}'), None);
    }

    #[test]
    fn test_smp() {
        assert_eq!(DT::of('\u{10000}'), None);
        assert_eq!(DT::of('\u{101fc}'), None);
        assert_eq!(DT::of('\u{101fd}'), None);
        assert_eq!(DT::of('\u{101fe}'), None);

        assert_eq!(DT::of('\u{1e000}'), None);

        assert_eq!(DT::of('\u{1e949}'), None);
        assert_eq!(DT::of('\u{1e94a}'), None);
        assert_eq!(DT::of('\u{1e94b}'), None);

        assert_eq!(DT::of('\u{1efff}'), None);

        // noncharacters
        assert_eq!(DT::of('\u{1fffe}'), None);
        assert_eq!(DT::of('\u{1ffff}'), None);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(DT::of('\u{30000}'), None);
        assert_eq!(DT::of('\u{40000}'), None);
        assert_eq!(DT::of('\u{50000}'), None);
        assert_eq!(DT::of('\u{60000}'), None);
        assert_eq!(DT::of('\u{70000}'), None);
        assert_eq!(DT::of('\u{80000}'), None);
        assert_eq!(DT::of('\u{90000}'), None);
        assert_eq!(DT::of('\u{a0000}'), None);
    }
}
