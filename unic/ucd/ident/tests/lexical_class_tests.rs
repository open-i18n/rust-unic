#![cfg(feature = "id")]
#![cfg(feature = "pattern")]

#[macro_use]
extern crate unic_char_range;
extern crate unic_ucd_ident;
extern crate unic_ucd_category;

#[macro_use]
extern crate matches;
extern crate regex;

use std::char;
use std::collections::BTreeSet;

use unic_ucd_category::GeneralCategory;
use unic_ucd_category::GeneralCategory::*;
use unic_ucd_ident::{is_id_start, is_id_continue};
use unic_ucd_ident::{is_pattern_syntax, is_pattern_whitespace};

use regex::Regex;

#[test]
/// ref: <http://www.unicode.org/reports/tr31/#Table_Lexical_Classes_for_Identifiers>
fn test_id_derivation() {
    let other_start: BTreeSet<char> = {
        Regex::new(
            r"(?xm)^
              ([[:xdigit:]]{4,6})        # low
              (?:..([[:xdigit:]]{4,6}))? # high
              \s+;\s+
              Other_ID_Start             # property
             "
        )
            .unwrap()
            .captures_iter(include_str!("../../../../data/ucd/PropList.txt"))
            .flat_map(|cap: regex::Captures| {
                let low = char::from_u32(u32::from_str_radix(&cap[1], 16).unwrap()).unwrap();
                let high = cap
                    .get(2)
                    .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .unwrap_or(low);
                chars!(low..=high)
            })
            .collect()
    };

    let other_continue: BTreeSet<char> = {
        Regex::new(
            r"(?xm)^
              ([[:xdigit:]]{4,6})        # low
              (?:..([[:xdigit:]]{4,6}))? # high
              \s+;\s+
              Other_ID_Continue          # property
             "
        )
            .unwrap()
            .captures_iter(include_str!("../../../../data/ucd/PropList.txt"))
            .flat_map(|cap: regex::Captures| {
                let low = char::from_u32(u32::from_str_radix(&cap[1], 16).unwrap()).unwrap();
                let high = cap
                    .get(2)
                    .map(|s| u32::from_str_radix(s.as_str(), 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .unwrap_or(low);
                chars!(low..=high)
            })
            .collect()
    };

    let is_id_start_derived = |ch| {
        let class = GeneralCategory::of(ch);
        (class.is_letter() || class == LetterNumber || other_start.contains(&ch))
            && !is_pattern_syntax(ch)
            && !is_pattern_whitespace(ch)
    };

    let is_id_continue_derived = |ch| {
        let class = GeneralCategory::of(ch);
        (matches!(class, NonspacingMark | SpacingMark | DecimalNumber | ConnectorPunctuation) ||
         is_id_start_derived(ch) ||
         other_continue.contains(&ch))
            && !is_pattern_syntax(ch)
            && !is_pattern_whitespace(ch)
    };

    for ch in chars!(..) {
        assert_eq!(is_id_start(ch), is_id_start_derived(ch));
        assert_eq!(is_id_continue(ch), is_id_continue_derived(ch));
    }
}
