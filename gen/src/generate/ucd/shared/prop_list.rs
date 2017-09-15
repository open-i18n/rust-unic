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
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use super::BINARY_PROPERTIES_REGEX;

#[derive(Clone, Debug, Default)]
pub struct PropList {
    ascii_hex_digit: BTreeSet<char>,
    bidi_control: BTreeSet<char>,
    dash: BTreeSet<char>,
    deprecated: BTreeSet<char>,
    diacritic: BTreeSet<char>,
    extender: BTreeSet<char>,
    hex_digit: BTreeSet<char>,
    // hyphen is deprecated
    ideographic: BTreeSet<char>,
    ids_binary_operator: BTreeSet<char>,
    ids_trinary_operator: BTreeSet<char>,
    join_control: BTreeSet<char>,
    logical_order_exception: BTreeSet<char>,
    noncharacter_code_point: BTreeSet<char>,
    other_alphabetic: BTreeSet<char>,
    other_default_ignorable_code_point: BTreeSet<char>,
    other_grapheme_extend: BTreeSet<char>,
    other_id_continue: BTreeSet<char>,
    other_id_start: BTreeSet<char>,
    other_lowercase: BTreeSet<char>,
    other_math: BTreeSet<char>,
    other_uppercase: BTreeSet<char>,
    pattern_syntax: BTreeSet<char>,
    pattern_white_space: BTreeSet<char>,
    prepended_concatenation_mark: BTreeSet<char>,
    quotation_mark: BTreeSet<char>,
    radical: BTreeSet<char>,
    regional_indicator: BTreeSet<char>,
    sentence_terminal: BTreeSet<char>,
    soft_dotted: BTreeSet<char>,
    terminal_punctuation: BTreeSet<char>,
    unified_ideograph: BTreeSet<char>,
    variation_selector: BTreeSet<char>,
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

pub fn read_prop_list() -> io::Result<PropList> {
    let mut file = File::open(Path::new("data/ucd/PropList.txt"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.parse().expect("Failed to parse PropList"))
}
