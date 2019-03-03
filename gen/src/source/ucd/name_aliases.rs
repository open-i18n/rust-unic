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

use crate::source::utils::read;

// String constants representing name alias types
// Note: The string corresponds to unic_ucd_name_aliases::NameAliasType enum cases
static TYPE_STR_CORRECTIONS: &str = "NameCorrections";
static TYPE_STR_CONTROLS: &str = "ControlCodeNames";
static TYPE_STR_ALTERNATES: &str = "AlternateNames";
static TYPE_STR_FIGMENTS: &str = "Figments";
static TYPE_STR_ABBREVIATIONS: &str = "NameAbbreviations";

lazy_static! {
    pub static ref NAME_ALIASES_DATA: NameAliasesData = {
        read("external/unicode/ucd/data/NameAliases.txt")
            .parse()
            .unwrap()
    };
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NameAliasesData {
    // A map from character to a vector of name alias type strings
    pub name_alias_types: BTreeMap<char, Vec<String>>,

    // Maps from character to a vector of name aliases
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
            )
            .unwrap();
        }

        let mut name_alias_types: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut corrections: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut controls: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut alternates: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut figments: BTreeMap<char, Vec<String>> = BTreeMap::default();
        let mut abbreviations: BTreeMap<char, Vec<String>> = BTreeMap::default();

        for capture in REGEX.captures_iter(str) {
            let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
            let chr = char::from_u32(code_point).unwrap();

            macro_rules! insert_or_append {
                ($map: ident, $value: expr) => {
                    if $map.contains_key(&chr) {
                        if let Some(values) = $map.get_mut(&chr) {
                            // Check for duplications because this macro is also used
                            // for inserting types strings to name_alias_types.
                            // In case of a character having multiple values for the
                            // same type, if we don't check for duplications, the type
                            // string would be inserted more than once.
                            if !values.contains(&$value) {
                                values.push($value);
                            }
                        }
                    } else {
                        $map.insert(chr, vec![$value]);
                    }
                };
            }

            let alias = capture[2].to_owned();
            match &capture[3] {
                "correction" => {
                    insert_or_append!(name_alias_types, TYPE_STR_CORRECTIONS.to_owned());
                    insert_or_append!(corrections, alias);
                }
                "control" => {
                    insert_or_append!(name_alias_types, TYPE_STR_CONTROLS.to_owned());
                    insert_or_append!(controls, alias);
                }
                "alternate" => {
                    insert_or_append!(name_alias_types, TYPE_STR_ALTERNATES.to_owned());
                    insert_or_append!(alternates, alias);
                }
                "figment" => {
                    insert_or_append!(name_alias_types, TYPE_STR_FIGMENTS.to_owned());
                    insert_or_append!(figments, alias);
                }
                "abbreviation" => {
                    insert_or_append!(name_alias_types, TYPE_STR_ABBREVIATIONS.to_owned());
                    insert_or_append!(abbreviations, alias);
                }
                _ => assert!(false, "Unexpected type label!"),
            }
        }

        Ok(NameAliasesData {
            name_alias_types,
            corrections,
            controls,
            alternates,
            figments,
            abbreviations,
        })
    }
}
