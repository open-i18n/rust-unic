use std::char;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use super::UnicodeVersion;

use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct IdnaDataLine {
    low: char,
    high: char,

    /// valid, ignored, mapped, deviation, disallowed,
    /// disallowed_STD3_valid, or disallowed_STD3_mapped
    status: String,

    /// Only present if the status is ignored, mapped, deviation, or disallowed_STD3_mapped.
    mapping: Option<Box<[char]>>,

    /// There are two values: NV8 and XV8.
    /// NV8 is only present if the status is valid but the character is excluded
    /// by IDNA2008 from all domain names for all versions of Unicode.
    /// XV8 is present when the character is excluded
    /// by IDNA2008 for the current version of Unicode.
    /// These are not normative values.
    idna_2008_status: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct IdnaMapping(Box<[IdnaDataLine]>);

impl IdnaMapping {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("idna_mapping.rsv"))?;
        let IdnaMapping(ref lines) = *self;
        writeln!(file, "&[")?;
        for line in lines.iter() {
            write!(
                file,
                "    ('{}', '{}', {}",
                line.low.escape_unicode(),
                line.high.escape_unicode(),
                match line.status.as_str() {
                    "valid" => "Valid",
                    "ignored" => "Ignored",
                    "mapped" => "Mapped",
                    "deviation" => "Deviation",
                    "disallowed" => "Disallowed",
                    "disallowed_STD3_valid" => "DisallowedStd3Valid",
                    "disallowed_STD3_mapped" => "DisallowedStd3Mapped",
                    _ => unreachable!(),
                },
            )?;
            if let Some(ref chars) = line.mapping {
                write!(file, "(&[")?;
                for char in chars.iter() {
                    write!(file, "'{}',", char.escape_unicode())?;
                }
                write!(file, "])")?;
            }
            writeln!(file, "),")?;
        }
        writeln!(file, "]")?;
        Ok(())
    }
}

impl FromStr for IdnaMapping {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)(?xm)^
                  ([[:xdigit:]]{4,6})                              # low code point
                  (?:\.\.([[:xdigit:]]{4,6}))?                     # high code point (option)
                  [[:space:]]*;[[:space:]]*
                  ([[:word:]]*)                                    # status
                  [[:space:]]*
                  (?:
                    ;[[:space:]]*
                    ([[:xdigit:]]{4,6}(?:\x20[[:xdigit:]]{4,6})*)? # mapping (option)
                    [[:space:]]*
                    (?:
                      ;[[:space:]]*
                      (NV8|XV8)                                    # IDNA2008 Status (option)
                      [[:space:]]*
                    )?
                  )?
                \#"
            ).unwrap();
        }

        // Initial capacity is an estimate of the number of data lines.
        let mut entries = Vec::with_capacity(0x2200);

        for capture in REGEX.captures_iter(str) {
            if let Some(low) = char::from_u32(u32::from_str_radix(&capture[1], 16).unwrap()) {
                entries.push(IdnaDataLine {
                    low,
                    high: capture
                        .get(2)
                        .map(|m| u32::from_str_radix(m.as_str(), 16).unwrap())
                        .map(|u| char::from_u32(u).unwrap())
                        .unwrap_or(low),
                    status: capture[3].to_owned(),
                    mapping: capture.get(4).map(|m| {
                        m.as_str()
                            .split(' ')
                            .map(|s| u32::from_str_radix(s, 16).unwrap())
                            .map(|u| char::from_u32(u).unwrap())
                            .collect::<Vec<_>>()
                            .into_boxed_slice()
                    }),
                    idna_2008_status: capture.get(5).map(|m| m.as_str().to_owned()),
                })
            }
        }

        Ok(IdnaMapping(entries.into_boxed_slice()))
    }
}

fn read_idna_data() -> io::Result<IdnaMapping> {
    let mut file = File::open(Path::new("data/idna/IdnaMappingTable.txt"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.parse().unwrap())
}

pub fn generate<P: AsRef<Path>>(dir: P, version: &UnicodeVersion) -> io::Result<()> {
    println!("> unic::ucd::idna::mapping::unicode_version.rsv");
    version.emit(&dir)?;
    println!(">>> Loading idna IdnaMappingTable");
    let idna_data = read_idna_data()?;
    println!("> unic::ucd::idna::maping::idna_mapping.rsv");
    idna_data.emit(&dir)?;
    Ok(())
}
