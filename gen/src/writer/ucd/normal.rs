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
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use source::ucd::derived_normalization_props::COMPOSITION_EXCLUSIONS;
use source::ucd::readme::UNICODE_VERSION;
use source::ucd::unicode_data::UNICODE_DATA;

use writer::common::emit_unicode_version;
use writer::utils::tables::{ToDirectCharTable, ToRangeCharSet, ToRangeCharTable};
use writer::utils::{capitalize, write};

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_general_category_mark(dir);
    emit_canonical_combining_class(dir);
    emit_canonical_decomposition_mapping(dir);
    emit_canonical_composition_mapping(dir);
    emit_compatibility_decomposition_mapping(dir);
}

fn emit_general_category_mark(dir: &Path) {
    let set: BTreeSet<char> = UNICODE_DATA
        .entries
        .iter()
        .filter(|x| matches!(x.general_category.as_str(), "Mn" | "Mc" | "Me"))
        .map(|x| x.character)
        .collect();

    write(dir, "general_category_mark.rsv", &set.to_range_char_set());
}

fn emit_canonical_combining_class(dir: &Path) {
    let map: BTreeMap<char, u8> = UNICODE_DATA
        .entries
        .iter()
        .filter(|x| x.canonical_combining_class != 0)
        .map(|x| (x.character, x.canonical_combining_class))
        .collect();

    write(
        dir,
        "canonical_combining_class_values.rsv",
        &map.to_range_char_table(|val, f| write!(f, "CanonicalCombiningClass({})", val)),
    );
}

fn decomposition_map<'a>() -> &'a BTreeMap<char, Box<[char]>> {
    lazy_static! {
        static ref MAP: BTreeMap<char, Box<[char]>> = {
            UNICODE_DATA
                .entries
                .iter()
                .filter(|x| x.decomposition_type.is_none() && x.decomposition_mapping.is_some())
                .map(|x| (x.character, x.decomposition_mapping.clone().unwrap()))
                .collect()
        };
    }
    &MAP
}

fn emit_canonical_decomposition_mapping(dir: &Path) {
    write(
        dir,
        "canonical_decomposition_mapping.rsv",
        &decomposition_map().to_direct_char_table(|raw, f| {
            write!(f, "&[")?;
            for char in raw.iter() {
                write!(f, "'{}',", char.escape_unicode())?;
            }
            write!(f, "]")
        }),
    );
}

fn emit_canonical_composition_mapping(dir: &Path) {
    let mut map: BTreeMap<char, Vec<(char, char)>> = BTreeMap::default();

    for (composed, decomposed) in decomposition_map() {
        if decomposed.len() == 1 {
            continue;
        }
        assert_eq!(decomposed.len(), 2);
        if COMPOSITION_EXCLUSIONS.set.contains(&composed) {
            continue;
        }
        let lead = decomposed[0];
        let follow = decomposed[1];
        map.entry(lead)
            .or_insert_with(|| Vec::with_capacity(1))
            .push((follow, *composed));
    }

    for value in map.values_mut() {
        value.sort_by_key(|it| it.0)
    }

    write(
        dir,
        "canonical_composition_mapping.rsv",
        &map.to_direct_char_table(|val, f| {
            write!(f, "CharDataTable::Direct(&[")?;
            for pair in val.iter() {
                write!(
                    f,
                    "('{}','{}'),",
                    pair.0.escape_unicode(),
                    pair.1.escape_unicode(),
                )?;
            }
            write!(f, "])")
        }),
    );
}

fn emit_compatibility_decomposition_mapping(dir: &Path) {
    let map: BTreeMap<char, (&str, &[char])> = UNICODE_DATA
        .entries
        .iter()
        .filter_map(|x| {
            if let Some(ref dt) = x.decomposition_type {
                if let Some(ref mapping) = x.decomposition_mapping {
                    return Some((x.character, (dt.as_str(), mapping.as_ref())));
                }
            }
            None
        })
        .collect();

    write(
        dir,
        "compatibility_decomposition_mapping.rsv",
        &map.to_direct_char_table(|val, f| {
            write!(f, "({}, &[", capitalize(&val.0.to_lowercase()))?;
            for char in val.1.iter() {
                write!(f, "'{}',", char.escape_unicode()).unwrap();
            }
            write!(f, "])")
        }),
    );
}
