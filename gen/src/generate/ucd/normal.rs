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
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use super::{UnicodeData, UnicodeDataEntry, UnicodeVersion};

use generate::PREAMBLE;
use generate::char_property::{ToBSearchSet, ToRangeBSearchMap, ToSingleBSearchMap};
use generate::capitalize;

use regex::Regex;

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

struct CompositionExclusions(BTreeSet<char>);

impl CompositionExclusions {
    fn contains(&self, ch: char) -> bool {
        self.0.contains(&ch)
    }
}

impl FromStr for CompositionExclusions {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^
                  ([[:xdigit:]]{4,6})
                  (?:\.\.([[:xdigit:]]{4,6}))?
                  [[:space:]]*;
                  \x20Full_Composition_Exclusion\x20
                \#",
            ).unwrap();
        }

        let mut exclusions = BTreeSet::default();
        for capture in REGEX.captures_iter(str) {
            let low = u32::from_str_radix(&capture[1], 16).unwrap();
            let high = capture
                .get(2)
                .map_or(low, |m| u32::from_str_radix(m.as_str(), 16).unwrap());
            for point in low..(high + 1) {
                if let Some(char) = char::from_u32(point) {
                    exclusions.insert(char);
                }
            }
        }

        Ok(CompositionExclusions(exclusions))
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
            if exclusions.contains(composed) {
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
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("compatibility_decomposition_mapping.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0.to_single_bsearch_map(|val, f| {
                write!(f, "({}, &[", capitalize(&val.0.to_lowercase()))?;
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
    println!(">>> Loading Composition Exclusions");
    let mut buffer = String::new();
    let mut file = File::open(Path::new("data/ucd/DerivedNormalizationProps.txt"))?;
    file.read_to_string(&mut buffer)?;
    let exclusions: CompositionExclusions = buffer.parse().unwrap();
    println!("> unic::ucd::normal::tables::canonical_composition_mapping");
    CanonicalCompositionData::from(&decomposition, &exclusions).emit(&dir)?;
    println!("> unic::ucd::normal::tables::compatibility_decomposition_mapping");
    CompatibilityDecompositionData::from(data.iter()).emit(&dir)?;
    Ok(())
}
