use std::char;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as WriteFmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeDataEntry, UnicodeVersion};

use generate::PREAMBLE;
use generate::char_property::{ToBSearchSet, ToRangeBSearchMap, ToSingleBSearchMap};

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
                .to_range_bsearch_map(|val| { format!("CanonicalCombiningClass({})", val) })
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
            self.0.to_single_bsearch_map(|val| {
                let mut s = String::from("&[");
                for char in val.iter() {
                    write!(s, "'{}',", char.escape_unicode()).unwrap();
                }
                s.push(']');
                s
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
                if let &Some(ref mapping) = decomposition_mapping {
                    map.insert(character, mapping.as_ref());
                }
            }
        }

        CanonicalDecompositionData(map)
    }
}

struct CompatibilityDecompositionData<'a>(BTreeMap<char, (&'a str, &'a [char])>);

impl<'a> CompatibilityDecompositionData<'a> {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("compatibility_decomposition_mapping"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0.to_single_bsearch_map(|val| {
                let mut s = format!("(\"{}\", &[", val.0);
                for char in val.1.iter() {
                    write!(s, "'{}',", char.escape_unicode()).unwrap();
                }
                s.push_str("])");
                s
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
            if let &Some(ref typ) = decomposition_type {
                if let &Some(ref mapping) = decomposition_mapping {
                    map.insert(character, (typ.as_str(), mapping.as_ref()));
                }
            }
        }

        CompatibilityDecompositionData(map)
    }
}

pub fn generate<P: AsRef<Path>>(
    dir: P,
    version: &UnicodeVersion,
    data: &UnicodeData,
) -> io::Result<()> {
    println!("> unic::ucd::normal::tables::unicode_version");
    version.emit(&dir)?;
    println!("> unic::ucd::normal::tables::general_category_mark.rsv");
    GeneralCategoryMarkData::from(data.iter()).emit(&dir)?;
    println!("> unic::ucd::normal::tables::canonical_combining_class_values.rsv");
    CanonicalCombiningClassData::from(data.iter()).emit(&dir)?;
    println!("> unic::ucd::normal::tables::canonical_decomposition_mapping.rsv");
    CanonicalDecompositionData::from(data.iter()).emit(&dir)?;
    println!("> unic::ucd::normal::tables::compatibility_decomposition_mapping.rsv");
    CompatibilityDecompositionData::from(data.iter())
        .emit(&dir)?;
    Ok(())
}
