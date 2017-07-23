use std::char;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::UnicodeDataEntry;

use generate::PREAMBLE;
use generate::char_property::char_map::*;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CategoryData<'a>(BTreeMap<char, &'a str>);

impl<'a> CategoryData<'a> {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let CategoryData(ref map) = *self;
        let mut file = File::create(dir.as_ref().join("general_category.rsv"))?;
        writeln!(file, "{}\n{}", PREAMBLE, map.to_bsearch_table_default())?;
        Ok(())
    }
}

impl<'a, I> From<I> for CategoryData<'a>
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut map = BTreeMap::<char, &str>::new();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref general_category, .. } in it {
            map.insert(character, general_category);
        }

        CategoryData(map)
    }
}

pub fn generate<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    super::read_unicode_version()?.emit(&dir)?;
    println!("> unicode_version");
    let unicode_data = super::read_unicode_data()?;
    let category_data = CategoryData::from(unicode_data.iter());
    category_data.emit(dir)?;
    println!("> general_category");
    Ok(())
}
