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
extern crate unic_ucd_utils;

use unic_ucd_normal::DecompositionType as DT;
use unic_ucd_utils::iter_all_chars;
use std::collections::HashSet as Set;
use std::{char, u32};


const DT_TEST_DATA: &'static str =
    include_str!("../../../../data/ucd/test/DecompositionTypeTest.txt");


#[derive(Debug)]
struct Fail {
    pub line_num: Option<usize>,
    pub char: char,
    pub exp_dt: Option<DT>,
    pub actual_dt: Option<DT>,
}

#[test]
fn test_decomposition_type_conformance() {
    let mut passed_num: u32 = 0;
    let mut failures: Vec<Fail> = Vec::new();
    let mut to_consider: Set<char> = iter_all_chars().collect();

    for (line_idx, line) in DT_TEST_DATA.lines().enumerate() {
        // Remove comments and trailing whitespace
        let line = if let Some(idx) = line.find('#') {
            &line[0..idx]
        } else {
            line
        }.trim();

        // Ignore empty lines
        if line.is_empty() {
            continue;
        }

        // State setting lines
        if line.starts_with('@') {
            panic!("State setting line in DecompositionTypeTest.txt; expected none");
        }

        // Data lines
        {
            let fields: Vec<_> = line.split(';').map(str::trim).collect();
            let input_range = {
                let range_text = fields[0];
                let mut range = range_text.split("..");
                let low = u32::from_str_radix(range.next().unwrap(), 16).unwrap();
                let high = range
                    .next()
                    .map(|c| u32::from_str_radix(c, 16).unwrap())
                    .unwrap_or(low);
                low..(high + 1)
            };
            let exp_dt = Some(get_dt_from_name(fields[1]));

            for codepoint in input_range {
                // TODO: We should be safe to use from_u32_unsafe
                if let Some(char) = char::from_u32(codepoint) {
                    let actual_dt = DT::of(char);
                    if actual_dt != exp_dt {
                        failures.push(Fail {
                            line_num: Some(line_idx + 1),
                            char,
                            exp_dt,
                            actual_dt,
                        });
                    } else {
                        passed_num += 1;
                    }
                    to_consider.remove(&char);
                }
            }
        }
    }

    // Defaults
    for char in to_consider {
        let exp_dt = None;
        let actual_dt = DT::of(char);
        if actual_dt != exp_dt {
            failures.push(Fail {
                line_num: None,
                char,
                exp_dt,
                actual_dt,
            });
        } else {
            passed_num += 1;
        }
    }

    // Report
    if !failures.is_empty() {
        // TODO: Show a list of failed cases when the number is less than 1K
        panic!(
            "{} test cases failed! ({} passed) {{\n\
             \n\
             0: {:?}\n\
             \n\
             ...\n\
             \n\
             {}: {:?}\n\
             \n\
             }}",
            failures.len(),
            passed_num,
            failures[0],
            failures.len() - 1,
            failures[failures.len() - 1],
        );
    }
}

fn get_dt_from_name(name: &str) -> DT {
    match name {
        "Canonical" | "Can" => DT::Canonical,
        "Compat" | "Com" => DT::Compat,
        "Circle" | "Enc" => DT::Circle,
        "Final" | "Fin" => DT::Final,
        "Font" => DT::Font,
        "Fraction" | "Fra" => DT::Fraction,
        "Initial" | "Init" => DT::Initial,
        "Isolated" | "Iso" => DT::Isolated,
        "Medial" | "Med" => DT::Medial,
        "Narrow" | "Nar" => DT::Narrow,
        "Nobreak" | "Nb" => DT::Nobreak,
        "None" => DT::None,
        "Small" | "Sml" => DT::Small,
        "Square" | "Sqr" => DT::Square,
        "Sub" => DT::Sub,
        "Super" | "Sup" => DT::Super,
        "Vertical" | "Vert" => DT::Vertical,
        "Wide" => DT::Wide,
        &_ => panic!("Invalid Decomposition_Type name: {}", name),
    }
}
