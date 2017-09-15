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

use reader::ucd::unicode_data::{UnicodeDataEntry, UNICODE_DATA};
use reader::ucd::derived_normalization_props::{CompositionExclusions, COMPOSITION_EXCLUSIONS};

use writer::utils::{capitalize, write};
use writer::utils::tables::{ToDirectCharTable, ToRangeCharSet, ToRangeCharTable};
use writer::ucd::unicode_version;


struct GeneralCategoryMarkData(BTreeSet<char>);

impl GeneralCategoryMarkData {
    fn emit(&self, dir: &Path) {
        let contents = self.0.to_range_char_set();
        write(dir, "general_category_mark.rsv", &contents);
    }
}

impl<'a, I> From<I> for GeneralCategoryMarkData
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut set = BTreeSet::new();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref general_category, .. } in it {
            if matches!(general_category.as_str(), "Mn" | "Mc" | "Me") {
                set.insert(character);
            }
        }

        GeneralCategoryMarkData(set)
    }
}

struct CanonicalCombiningClassData(BTreeMap<char, u8>);

impl CanonicalCombiningClassData {
    fn emit(&self, dir: &Path) {
        let contents = self.0
            .to_range_char_table(|val, f| write!(f, "CanonicalCombiningClass({})", val));
        write(dir, "canonical_combining_class_values.rsv", &contents);
    }
}

impl<'a, I> From<I> for CanonicalCombiningClassData
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut map = BTreeMap::default();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref canonical_combining_class, .. } in it {
            if *canonical_combining_class != 0 {
                map.insert(character, *canonical_combining_class);
            }
        }

        CanonicalCombiningClassData(map)
    }
}

struct CanonicalDecompositionData<'a>(BTreeMap<char, &'a [char]>);

impl<'a> CanonicalDecompositionData<'a> {
    fn emit(&self, dir: &Path) {
        let contents = self.0.to_direct_char_table(|val, f| {
            write!(f, "&[")?;
            for char in val.iter() {
                write!(f, "'{}',", char.escape_unicode())?;
            }
            write!(f, "]")
        });
        write(dir, "canonical_decomposition_mapping.rsv", &contents);
    }
}

impl<'a, I> From<I> for CanonicalDecompositionData<'a>
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut map = BTreeMap::default();

        for &UnicodeDataEntry {
            character,
            ref decomposition_type,
            ref decomposition_mapping,
            ..
        } in it
        {
            if decomposition_type.is_none() {
                if let Some(ref mapping) = *decomposition_mapping {
                    map.insert(character, mapping.as_ref());
                }
            }
        }

        CanonicalDecompositionData(map)
    }
}

struct CanonicalCompositionData(BTreeMap<char, Vec<(char, char)>>);

impl CanonicalCompositionData {
    fn emit(&self, dir: &Path) {
        let contents = self.0.to_direct_char_table(|val, f| {
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
        });
        write(dir, "canonical_composition_mapping.rsv", &contents);
    }

    fn from<'a>(
        decomposition: &CanonicalDecompositionData<'a>,
        exclusions: &CompositionExclusions,
    ) -> CanonicalCompositionData {
        let mut map = BTreeMap::default();
        let &CanonicalDecompositionData(ref decomposition_map) = decomposition;

        for (&composed, decomposed) in decomposition_map {
            if decomposed.len() == 1 {
                continue;
            }
            assert_eq!(decomposed.len(), 2);
            if exclusions.0.contains(&composed) {
                continue;
            }
            let lead = decomposed[0];
            let follow = decomposed[1];
            map.entry(lead)
                .or_insert_with(|| Vec::with_capacity(1))
                .push((follow, composed));
        }

        for value in map.values_mut() {
            value.sort_by_key(|it| it.0)
        }

        CanonicalCompositionData(map)
    }
}

struct CompatibilityDecompositionData<'a>(BTreeMap<char, (&'a str, &'a [char])>);

impl<'a> CompatibilityDecompositionData<'a> {
    fn emit(&self, dir: &Path) {
        let contents = self.0.to_direct_char_table(|val, f| {
            write!(f, "({}, &[", capitalize(&val.0.to_lowercase()))?;
            for char in val.1.iter() {
                write!(f, "'{}',", char.escape_unicode()).unwrap();
            }
            write!(f, "])")
        });
        write(dir, "compatibility_decomposition_mapping.rsv", &contents);
    }
}

impl<'a, I> From<I> for CompatibilityDecompositionData<'a>
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut map = BTreeMap::default();

        for &UnicodeDataEntry {
            character,
            ref decomposition_type,
            ref decomposition_mapping,
            ..
        } in it
        {
            if let Some(ref typ) = *decomposition_type {
                if let Some(ref mapping) = *decomposition_mapping {
                    map.insert(character, (typ.as_str(), mapping.as_ref()));
                }
            }
        }

        CompatibilityDecompositionData(map)
    }
}


pub fn generate(dir: &Path) {
    unicode_version::emit(dir);

    GeneralCategoryMarkData::from(UNICODE_DATA.0.iter()).emit(dir);
    CanonicalCombiningClassData::from(UNICODE_DATA.0.iter()).emit(dir);

    let decomposition = CanonicalDecompositionData::from(UNICODE_DATA.0.iter());
    decomposition.emit(dir);

    CanonicalCompositionData::from(&decomposition, &COMPOSITION_EXCLUSIONS).emit(dir);
    CompatibilityDecompositionData::from(UNICODE_DATA.0.iter()).emit(dir);
}
