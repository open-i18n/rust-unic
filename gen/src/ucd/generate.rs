// TODO: Break this file up

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::{char, u32};

use super::ucd_data_dir;
use utils::{common, rustout};

use regex::Regex;

/// file!() but with UNIX path separators, even on Windows
const SCRIPT: &'static str = "gen/src/ucd/generate.rs";

lazy_static! {
    static ref CORE_TABLES: &'static Path = Path::new("components/ucd/core/src/tables");
    static ref AGE_TABLES: &'static Path = Path::new("components/ucd/age/src/tables");
    static ref BIDI_TABLES: &'static Path = Path::new("components/ucd/bidi/src/tables");
    static ref CATEGORY_TABLES: &'static Path = Path::new("components/ucd/category/src/tables");
    static ref NORMAL_TABLES: &'static Path = Path::new("components/ucd/normal/src/tables");
    static ref NORMAL_TEST_TABLES: &'static Path = Path::new("components/normal/test/tables");
}

// == VERSION == //

lazy_static! {
    static ref UNICODE_VERSION: (u16, u16, u16) = {
        let mut readme = File::open(ucd_data_dir().join("ReadMe.txt")).unwrap();
        let mut buffer = String::new();
        readme.read_to_string(&mut buffer).unwrap();
        let pattern = Regex::new(r"for Version (\d+)\.(\d+)\.(\d+)").unwrap();
        let captures = pattern.captures(&buffer).unwrap();
        (
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
            captures.get(3).unwrap().as_str().parse().unwrap(),
        )
    };
}

fn emit_unicode_version(dir: &Path) {
    // TODO: Crashes when src/tables/ doesn't exist!
    let mut file = File::create(dir.join("unicode_version.rsv")).unwrap();
    rustout::emit_value(SCRIPT, &mut file, &UNICODE_VERSION, |triple| {
        format!(
            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
            triple.0,
            triple.1,
            triple.2
        )
    }).unwrap();
}

// == Shared == //

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct UnicodeDataEntry {
    pub codepoint: u32,
    pub name: String,
    pub general_category: String,
    pub canonical_combining_class: String,
    pub bidi_class: String,
    pub decomposition: String,
    pub deci: String,
    pub digit: String,
    pub num: String,
    pub bidi_mirrored: String,
    pub unicode_1_name: String,
    pub iso_comment: String,
    pub simple_uppercase_mapping: String,
    pub simple_lowercase_mapping: String,
    pub simple_titlecase_mapping: String,
}

impl UnicodeDataEntry {
    fn from<I: IntoIterator<Item = String>>(iterator: I) -> Self {
        let mut iterator = iterator.into_iter();
        UnicodeDataEntry {
            codepoint: u32::from_str_radix(&iterator.next().unwrap(), 16).unwrap(),
            name: iterator.next().unwrap(),
            general_category: iterator.next().unwrap(),
            canonical_combining_class: iterator.next().unwrap(),
            bidi_class: iterator.next().unwrap(),
            decomposition: iterator.next().unwrap(),
            deci: iterator.next().unwrap(),
            digit: iterator.next().unwrap(),
            num: iterator.next().unwrap(),
            bidi_mirrored: iterator.next().unwrap(),
            unicode_1_name: iterator.next().unwrap(),
            iso_comment: iterator.next().unwrap(),
            simple_uppercase_mapping: iterator.next().unwrap(),
            simple_lowercase_mapping: iterator.next().unwrap(),
            simple_titlecase_mapping: iterator.next().unwrap(),
        }
    }
}

lazy_static! {
    static ref UNICODE_DATA: HashMap<u32, UnicodeDataEntry> = {
        let mut unicode_data = HashMap::default();
        let mut range_start: Option<u32> = None;
        let file = File::open(ucd_data_dir().join("UnicodeData.txt")).unwrap();
        for line in BufReader::new(file).lines() {
            let data: Vec<_> = line.unwrap().split(';').map(String::from).collect();
            if data.len() != 15 { continue }
            let mut data = UnicodeDataEntry::from(data);
            // skip surrogates
            if let Some(_) = char::from_u32(data.codepoint) {
                if let Some(start) = range_start {
                    data.name = String::from("");
                    let end = data.codepoint+1;
                    for i in start..end {
                        data.codepoint = i;
                        unicode_data.insert(data.codepoint, data.clone());
                    }
                    range_start = None;
                }
                if data.name.ends_with(", First>") {
                    range_start = Some(data.codepoint);
                } else {
                    unicode_data.insert(data.codepoint, data);
                }
            }
        }
        unicode_data
    };
}

// == General_Category == //

lazy_static! {
    // Extracted from TR44 Table 12 <http://www.unicode.org/reports/tr44/#GC_Values_Table>
    static ref CATEGORY_EXPANSION: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::default();
        map.insert("Lu", vec!["Lu", "LC", "L"]);
        map.insert("Ll", vec!["Ll", "LC", "L"]);
        map.insert("Lt", vec!["Lt", "LC", "L"]);
        map.insert("Lm", vec!["Lm", "L"]);
        map.insert("Lo", vec!["Lo", "L"]);
        map.insert("Mn", vec!["Mn", "M"]);
        map.insert("Mc", vec!["Mc", "M"]);
        map.insert("Me", vec!["Me", "M"]);
        map.insert("Nd", vec!["Nd", "N"]);
        map.insert("Nl", vec!["Nl", "N"]);
        map.insert("No", vec!["No", "N"]);
        map.insert("Pc", vec!["Pc", "P"]);
        map.insert("Pd", vec!["Pd", "P"]);
        map.insert("Ps", vec!["Ps", "P"]);
        map.insert("Pe", vec!["Pe", "P"]);
        map.insert("Pi", vec!["Pi", "P"]);
        map.insert("Pf", vec!["Pf", "P"]);
        map.insert("Po", vec!["Po", "P"]);
        map.insert("Sm", vec!["Sm", "S"]);
        map.insert("Sc", vec!["Sc", "S"]);
        map.insert("Sk", vec!["Sk", "S"]);
        map.insert("So", vec!["So", "S"]);
        map.insert("Zs", vec!["Zs", "Z"]);
        map.insert("Zl", vec!["Zl", "Z"]);
        map.insert("Zp", vec!["Zp", "Z"]);
        map.insert("Cc", vec!["Cc", "C"]);
        map.insert("Cf", vec!["Cf", "C"]);
        map.insert("Cs", vec!["Cs", "C"]);
        map.insert("Co", vec!["Co", "C"]);
        map.insert("Cn", vec!["Cn", "C"]);
        map
    };
}

// == Age ==

lazy_static! {
    static ref AGE_VALUES: Vec<(u32, u32, String)> = {
        let mut age_groups: HashMap<String, Vec<(u32, u32)>> = HashMap::default();
        let file = File::open(ucd_data_dir().join("DerivedAge.txt")).unwrap();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();

            // Remove comments
            let line = line.split('#').next().unwrap().trim();

            // Skip empty lines
            if line.len() == 0 {
                continue;
            }

            // Parse data
            let ((first, last), (major, micro)) = {
                let mut it = line.split(';');
                (
                    {
                        let mut it = it.next().unwrap().trim().split("..");
                        let first = it.next().unwrap();
                        let last = it.next().unwrap_or(first);
                        (
                            u32::from_str_radix(first, 16).unwrap(),
                            u32::from_str_radix(last, 16).unwrap(),
                        )
                    },
                    {
                        let mut it = it.next().unwrap().trim().split('.');
                        (it.next().unwrap(), it.next().unwrap())
                    },
                )
            };

            // Skip surrogate codepoints
            if (first, last) == (0xD800, 0xDFFF) {
                continue;
            }

            let age = format!(
                "Assigned(UnicodeVersion {{ major: {}, minor: {}, micro: 0 }})",
                major, micro,
            );

            age_groups.entry(age).or_insert_with(|| vec![]).push((first, last));
        }

        // Consolidate ranges
        for group in age_groups.values_mut() {
            *group = ranges_from_codepoints(
                codepoints_from_ranges(group.clone()),
            )
        }

        range_value_from_ranges(age_groups)
    };
}

fn emit_age_tables(dir: &Path) {
    let mut file = File::create(dir.join("age_values.rsv")).unwrap();
    rustout::emit_value_range_bsearch_table(SCRIPT, &mut file, AGE_VALUES.iter()).unwrap();
}

// == Bidi == //

lazy_static! {
    // TODO: bidi_class_groups: HashMap<&str, Vec<32>>
    // Requires changing range_value_from_codepoints to take HashMap<AsRef<str>, Vec<u32>>
    static ref BIDI_CLASS_VALUES: Vec<(u32, u32, String)> = {
        let mut bidi_class_groups: HashMap<String, Vec<u32>> = HashMap::default();

        for &UnicodeDataEntry { codepoint, ref bidi_class, .. } in UNICODE_DATA.values() {
            bidi_class_groups.entry(bidi_class.clone()).or_insert_with(|| vec![]).push(codepoint);
        }

        // Default Bidi_Class from unassigned codepoints
        // <http://www.unicode.org/Public/UNIDATA/extracted/DerivedBidiClass.txt>
        let default_ranges = &[
            // default to AL
            (0x0600, 0x07BF, "AL"),
            (0x08A0, 0x08FF, "AL"),
            (0xFB50, 0xFDCF, "AL"),
            (0xFDF0, 0xFDFF, "AL"),
            (0xFE70, 0xFEFF, "AL"),
            (0x1EE00, 0x1EEFF, "AL"),
            // default to R
            (0x0590, 0x05FF, "R"),
            (0x07C0, 0x089F, "R"),
            (0xFB1D, 0xFB4F, "R"),
            (0x10800, 0x10FFF, "R"),
            (0x1E800, 0x1EDFF, "R"),
            (0x1EF00, 0x1EFFF, "R"),
            // default to ET
            (0x20A0, 0x20CF, "ET"),
        ];

        for &(start, end, default) in default_ranges {
            for codepoint in start..(end+1) {
                if !UNICODE_DATA.contains_key(&codepoint) {
                    bidi_class_groups.get_mut(default).unwrap().push(codepoint);
                }
            }
        }

        range_value_from_codepoints(bidi_class_groups)
    };
}

fn emit_bidi_class_tables(dir: &Path) {
    let mut file = File::create(dir.join("bidi_class_values.rsv")).unwrap();
    rustout::emit_value_range_bsearch_table(SCRIPT, &mut file, BIDI_CLASS_VALUES.iter()).unwrap()
}

// == Category == //

lazy_static! {
    static ref GENERAL_CATEGORY_MAPPING: Vec<(u32, u32, String)> = {
        let mut general_category_mapping = HashMap::default();

        for &UnicodeDataEntry { codepoint, ref general_category, .. } in UNICODE_DATA.values() {
            general_category_mapping
                .entry(general_category.clone())
                .or_insert_with(|| vec![])
                .push(codepoint)
        }

        range_value_from_codepoints(general_category_mapping)
    };
}

fn emit_general_category_tables(dir: &Path) {
    let mut file = File::create(dir.join("general_category.rsv")).unwrap();
    rustout::emit_value_range_bsearch_table(SCRIPT, &mut file, GENERAL_CATEGORY_MAPPING.iter())
        .unwrap();
}

// == Miscellaneous == //

fn range_value_from_codepoints(groups: HashMap<String, Vec<u32>>) -> Vec<(u32, u32, String)> {
    let mut list: Vec<_> = groups
        .into_iter()
        .flat_map(|(str, codepoints)| {
            ranges_from_codepoints(codepoints)
                .into_iter()
                .map(move |range| (range.0, range.1, str.to_string()))
        })
        .collect();
    list.sort_by_key(|triple| triple.0);
    list
}

fn range_value_from_ranges(groups: HashMap<String, Vec<(u32, u32)>>) -> Vec<(u32, u32, String)> {
    let mut list: Vec<_> = groups
        .into_iter()
        .flat_map(|(str, ranges)| {
            ranges
                .into_iter()
                .map(move |range| (range.0, range.1, str.to_string()))
        })
        .collect();
    list.sort_by_key(|triple| triple.0);
    list
}

fn ranges_from_codepoints(mut codepoints: Vec<u32>) -> Vec<(u32, u32)> {
    if codepoints.len() == 0 {
        return Vec::new();
    }
    let mut ranges = vec![];
    codepoints.sort();
    codepoints.dedup();
    let mut start = codepoints.remove(0);
    let mut end = start;
    for codepoint in codepoints {
        assert!(codepoint > end);
        if codepoint == end + 1 {
            end = codepoint;
        } else {
            ranges.push((start, end));
            start = codepoint;
            end = codepoint;
        }
    }
    ranges.push((start, end));
    ranges
}

fn codepoints_from_ranges<I>(ranges: I) -> Vec<u32>
where
    I: IntoIterator<Item = (u32, u32)>,
{
    ranges
        .into_iter()
        .flat_map(|range| range.0..(range.1 + 1))
        .collect()
}

// == MAIN == //

pub fn run() {
    for &dir in &[*CORE_TABLES, *AGE_TABLES, *BIDI_TABLES, *NORMAL_TABLES, *NORMAL_TEST_TABLES] {
        common::cleanup_rsv(dir).unwrap();
    }

    // Core
    emit_unicode_version(*CORE_TABLES);

    // Age
    emit_unicode_version(*AGE_TABLES);
    emit_age_tables(*AGE_TABLES);

    // Bidi
    emit_unicode_version(*BIDI_TABLES);
    emit_bidi_class_tables(*BIDI_TABLES);

    // Category
    emit_unicode_version(*CATEGORY_TABLES);
    emit_general_category_tables(*CATEGORY_TABLES);
}
