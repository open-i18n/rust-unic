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
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeDataEntry, UnicodeVersion};

use generate::PREAMBLE;
use generate::char_property::{ToBSearchSet, ToRangeBSearchMap, ToSingleBSearchMap};
use generate::capitalize;

struct GeneralCategoryMarkData(BTreeSet<char>);

impl GeneralCategoryMarkData {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("general_category_mark.rsv"))?;
        writeln!(file, "{}\n{}", PREAMBLE, self.0.to_bsearch_set())
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
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("canonical_combining_class_values.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0
                .to_range_bsearch_map(|val, f| { write!(f, "CanonicalCombiningClass({})", val) })
        )
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
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("canonical_decomposition_mapping.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0.to_single_bsearch_map(|val, f| {
                write!(f, "&[")?;
                for char in val.iter() {
                    write!(f, "'{}',", char.escape_unicode())?;
                }
                write!(f, "]")
            }),
        )
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
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("canonical_composition_mapping.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0.to_single_bsearch_map(|val, f| {
                write!(f, "&[")?;
                for pair in val.iter() {
                    write!(
                        f,
                        "('{}','{}'),",
                        pair.0.escape_unicode(),
                        pair.1.escape_unicode(),
                    )?;
                }
                write!(f, "]")
            }),
        )
    }
}

impl<'a, 'b> From<&'a CanonicalDecompositionData<'b>> for CanonicalCompositionData {
    fn from(decomposition: &CanonicalDecompositionData<'b>) -> Self {
        let mut map = BTreeMap::default();
        let &CanonicalDecompositionData(ref decomposition_map) = decomposition;

        for (&composed, decomposed) in decomposition_map {
            if decomposed.len() == 1 {
                continue;
            }
            assert_eq!(decomposed.len(), 2);
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
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("compatibility_decomposition_mapping.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0.to_single_bsearch_map(|val, f| {
                write!(f, "({}, &[", capitalize(val.0))?;
                for char in val.1.iter() {
                    write!(f, "'{}',", char.escape_unicode()).unwrap();
                }
                write!(f, "])")
            }),
        )
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

/// Generate tables for the ucd-normal crate
pub fn generate<P: AsRef<Path>>(
    dir: P,
    version: &UnicodeVersion,
    data: &UnicodeData,
) -> io::Result<()> {
    println!("> unic::ucd::normal::tables::unicode_version");
    version.emit(&dir)?;
    println!("> unic::ucd::normal::tables::general_category_mark");
    GeneralCategoryMarkData::from(data.iter()).emit(&dir)?;
    println!("> unic::ucd::normal::tables::canonical_combining_class_values");
    CanonicalCombiningClassData::from(data.iter()).emit(&dir)?;
    println!("> unic::ucd::normal::tables::canonical_decomposition_mapping");
    let decomposition = CanonicalDecompositionData::from(data.iter());
    decomposition.emit(&dir)?;
    println!("> unic::ucd::normal::tables::canonical_composition_mapping");
    CanonicalCompositionData::from(&decomposition).emit(&dir)?;
    println!("> unic::ucd::normal::tables::compatibility_decomposition_mapping");
    CompatibilityDecompositionData::from(data.iter())
        .emit(&dir)?;
    Ok(())
}
