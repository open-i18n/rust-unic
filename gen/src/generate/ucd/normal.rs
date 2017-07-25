use std::char;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as WriteFmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeVersion};

use generate::PREAMBLE;
use generate::char_property::{ToBSearchSet, ToRangeBSearchMap, ToSingleBSearchMap};

struct GeneralCategoryMarkData(BTreeSet<char>);

impl GeneralCategoryMarkData {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("general_category_mark.rsv"))?;
        writeln!(file, "{}\n{}", PREAMBLE, self.0.to_bsearch_set())
    }
}

struct CanonicalCombiningClassData(BTreeMap<char, u8>);

impl CanonicalCombiningClassData {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("canonical_combining_class_values.rsv"))?;
        writeln!(file, "{}\n{}", PREAMBLE, self.0.to_range_bsearch_map(|val| {
            format!("CanonicalCombiningClass({})", val)
        }))
    }
}

struct CanonicalDecompositionData(BTreeMap<char, Box<[char]>>);

impl CanonicalDecompositionData {
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

struct CompatibilityDecompositionData<'a>(BTreeMap<char, (&'a str, Box<[char]>)>);

impl<'a> CompatibilityDecompositionData<'a> {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("compatibility_decomposition_mapping"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0.to_single_bsearch_map(|val| {
                let mut s = String::from("(\"");
                write!(s, "(\"{}\",&[", val.0).unwrap();
                for char in val.1.iter() {
                    write!(s, "'{}',", char.escape_unicode()).unwrap();
                }
                s.push_str("])");
                s
            }),
        )
    }
}

#[allow(unreachable_code)]
pub fn generate<P: AsRef<Path>>(
    dir: P,
    version: &UnicodeVersion,
    data: &UnicodeData,
) -> io::Result<()>
{
    version.emit(&dir)?;
    println!("> unic::ucd::normal::tables::unicode_version");
    let general_category_mark_data: GeneralCategoryMarkData = unimplemented!();
    general_category_mark_data.emit(&dir)?;
    println!(": unic::ucd::normal::tables::general_category_mark.rsv");
    let canonical_combining_class_data: CanonicalCombiningClassData = unimplemented!();
    canonical_combining_class_data.emit(&dir)?;
    println!("> unic::ucd::normal::tables::canonical_combining_class_values.rsv");
    let canonical_decomposition_data: CanonicalDecompositionData = unimplemented!();
    canonical_decomposition_data.emit(&dir)?;
    println!("> unic::ucd::normal::tables::canonical_decomposition_mapping.rsv");
    let compatibility_decomposition_data: CompatibilityDecompositionData = unimplemented!();
    compatibility_decomposition_data.emit(&dir)?;
    println!("> unic::ucd::normal::tables::compatibility_decomposition_mapping.rsv");
    Ok(())
}