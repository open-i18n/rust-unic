// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::char;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::vec::Vec;

use regex::Regex;

use source::utils::read;

lazy_static! {
    pub static ref NAME_ALIASES_DATA: NameAliasesData =
        { read("data/ucd/NameAliases.txt").parse().unwrap() };
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NameAliasesData {
    pub corrections: BTreeMap<char, Vec<String>>,
    pub controls: BTreeMap<char, Vec<String>>,
    pub alternates: BTreeMap<char, Vec<String>>,
    pub figments: BTreeMap<char, Vec<String>>,
    pub abbreviations: BTreeMap<char, Vec<String>>,
}

impl FromStr for NameAliasesData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^              # every line
                  ([[:xdigit:]]{4,6}) # codepoint
                  ;([[:alnum:]\ \-]*) # alias
                  ;([[:alpha:]]*)     # type
                ",
            ).unwrap();
        }

        let mut corrections: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut controls: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut alternates: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut figments: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut abbreviations: BTreeMap<char, Vec<String>> = BTreeMap::default();

        for capture in REGEX.captures_iter(str) {
            let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
            let chr = char::from_u32(code_point).unwrap();

            let alias = capture[2].to_owned();

            macro_rules! insert_or_append {
                ($map: ident) => {
                    if $map.contains_key(&chr) {
                        if let Some(aliases) = $map.get_mut(&chr) {
                            aliases.push(alias);
                        }
                    } else {
                        $map.insert(chr, vec![alias]);
                    }
                }
            }

            match &capture[3] {
                "correction" => insert_or_append!(corrections),
                "control" => insert_or_append!(controls),
                "alternate" => insert_or_append!(alternates),
                "figment" => insert_or_append!(figments),
                "abbreviation" => insert_or_append!(abbreviations),
                _ => assert!(false, "Unexpected type label!"),
            }
        }

        Ok(NameAliasesData {
            corrections,
            controls,
            alternates,
            figments,
            abbreviations
        })
    }
}
