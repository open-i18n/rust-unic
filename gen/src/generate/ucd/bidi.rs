use std::char;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::UnicodeDataEntry;

use generate::PREAMBLE;
use generate::char_property::char_map::*;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BidiData<'a>(BTreeMap<char, &'a str>);

impl<'a> BidiData<'a> {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let BidiData(ref map) = *self;
        let mut file = File::create(dir.as_ref().join("bidi_class_values.rsv"))?;
        writeln!(file, "{}\n{}", PREAMBLE, map.to_bsearch_table_default())?;
        Ok(())
    }
}

impl<'a, I> From<I> for BidiData<'a>
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
            (0x1EE00, 0x1EEFF, "AL"),
            (0x0590, 0x05FF, "R"),
            (0x07C0, 0x089F, "R"),
            (0xFB1D, 0xFB4F, "R"),
            (0x10800, 0x10FFF, "R"),
            (0x1E800, 0x1EDFF, "R"),
            (0x1EF00, 0x1EFFF, "R"),
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

        BidiData(map)
    }
}

pub fn generate<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    super::UNICODE_VERSION.emit(&dir)?;
    let unicode_data = super::read_unicode_data()?;
    let bidi_data = BidiData::from(unicode_data.iter());
    bidi_data.emit(dir)?;
    Ok(())
}
