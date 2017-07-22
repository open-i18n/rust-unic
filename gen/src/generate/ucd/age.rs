use std::char;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use generate::PREAMBLE;
use generate::char_property::char_map::*;

use regex::Regex;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct AgeData(BTreeMap<char, String>);

impl AgeData {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let AgeData(ref map) = *self;
        let mut file = File::create(dir.as_ref().join("age_values.rsv"))?;
        writeln!(file, "{}\n{}", PREAMBLE, map.to_bsearch_table_default())?;
        Ok(())
    }

    fn read<R: Read>(mut reader: R) -> io::Result<AgeData> {
        let regex = Regex::new(
            r"(?xm)^                     # every line
              ([[:xdigit:]]{4,6})        # range start
              (?:..([[:xdigit:]]{4,6}))? # range end (option)
              [[:blank:]]*;[[:blank:]]*  # separator
              ([[:digit:]])              # major version
              \.([[:digit:]])            # minor version
              (?:\.([[:digit:]]))?       # micro version (option)
            ",
        ).unwrap();
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        let captures = regex.captures_iter(&buffer);

        let mut age_data = BTreeMap::default();
        for capture in captures {
            let start = u32::from_str_radix(&capture[1], 16).unwrap();
            let end = capture
                .get(2)
                .map_or(start, |m| u32::from_str_radix(m.as_str(), 16).unwrap());
            let major = capture[3].parse::<u16>().unwrap();
            let minor = capture[4].parse::<u16>().unwrap();
            let micro = capture
                .get(5)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());

            for point in start..(end + 1) {
                if let Some(char) = char::from_u32(point) {
                    age_data.insert(
                        char,
                        format!(
                            "Assigned(UnicodeVersion {{ major: {}, minor: {}, micro: {} }})",
                            major,
                            minor,
                            micro,
                        ),
                    );
                }
            }
        }

        Ok(AgeData(age_data))
    }
}

pub fn generate<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    super::UNICODE_VERSION.emit(&dir)?;
    let derived_age = File::open(Path::new("data/ucd/DerivedAge.txt"))?;
    let age_data = AgeData::read(derived_age)?;
    age_data.emit(dir)?;
    Ok(())
}
