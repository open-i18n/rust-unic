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
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeDataEntry, UnicodeVersion};

use generate::PREAMBLE;
use generate::tables::{ToRangeCharSet, ToRangeCharTable};


// == Bidi_Class ==

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BidiClassData<'a>(BTreeMap<char, &'a str>);

impl<'a> BidiClassData<'a> {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let BidiClassData(ref map) = *self;
        let mut file = File::create(dir.as_ref().join("bidi_class_values.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            map.to_range_char_table(Display::fmt)
        )?;
        Ok(())
    }
}

impl<'a, I> From<I> for BidiClassData<'a>
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        // Default Bidi_Class for unassigned codepoints
        // <http://www.unicode.org/Public/UNIDATA/extracted/DerivedBidiClass.txt>
        let defaults = &[
            (0x0600, 0x07BF, "AL"),
            (0x08A0, 0x08FF, "AL"),
            (0xFB50, 0xFDCF, "AL"),
            (0xFDF0, 0xFDFF, "AL"),
            (0xFE70, 0xFEFF, "AL"),
            (0x1_EE00, 0x1_EEFF, "AL"),
            (0x0590, 0x05FF, "R"),
            (0x07C0, 0x089F, "R"),
            (0xFB1D, 0xFB4F, "R"),
            (0x1_0800, 0x1_0FFF, "R"),
            (0x1_E800, 0x1_EDFF, "R"),
            (0x1_EF00, 0x1_EFFF, "R"),
            (0x20A0, 0x20CF, "ET"),
        ];
        let mut map = BTreeMap::<char, &str>::new();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref bidi_class, .. } in it {
            map.insert(character, bidi_class);
        }

        for &(start, end, default) in defaults {
            for codepoint in start..(end + 1) {
                if let Some(c) = char::from_u32(codepoint) {
                    map.entry(c).or_insert(default);
                }
            }
        }

        BidiClassData(map)
    }
}


// == Bidi_Mirrored ==

struct BidiMirroredData(BTreeSet<char>);

impl BidiMirroredData {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("bidi_mirrored.rsv"))?;
        writeln!(file, "{}\n{}", PREAMBLE, self.0.to_range_char_set())
    }
}

impl<'a, I> From<I> for BidiMirroredData
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut set = BTreeSet::new();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, bidi_mirrored, .. } in it {
            if bidi_mirrored {
                set.insert(character);
            }
        }

        BidiMirroredData(set)
    }
}

/// Generate tables for the ucd-bidi crate
pub fn generate<P: AsRef<Path>>(
    dir: P,
    version: &UnicodeVersion,
    data: &UnicodeData,
) -> io::Result<()> {
    println!("> unic::ucd::bidi::tables::unicode_version");
    version.emit(&dir)?;
    println!("> unic::ucd::bidi::tables::bidi_class_values");
    BidiClassData::from(data.iter()).emit(&dir)?;
    BidiMirroredData::from(data.iter()).emit(&dir)?;
    Ok(())
}
