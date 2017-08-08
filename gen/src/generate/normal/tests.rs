use std::char;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use generate::PREAMBLE;

use regex::Regex;

struct NormalizationTest {
    source: Box<[char]>,
    nfc: Box<[char]>,
    nfd: Box<[char]>,
    nfkc: Box<[char]>,
    nfkd: Box<[char]>,
}

struct NormalizationTests(Box<[NormalizationTest]>);

impl NormalizationTests {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("conformance_tests_data.rsv"))?;
        let tests = &self.0;
        writeln!(file, "{}", PREAMBLE)?;
        writeln!(file, "&[")?;
        for test in tests.iter() {
            write!(file, "    (\"")?;
            for char in test.source.iter() {
                write!(file, "{}", char.escape_unicode())?;
            }
            write!(file, "\", \"")?;
            for char in test.nfc.iter() {
                write!(file, "{}", char.escape_unicode())?;
            }
            write!(file, "\", \"")?;
            for char in test.nfd.iter() {
                write!(file, "{}", char.escape_unicode())?;
            }
            write!(file, "\", \"")?;
            for char in test.nfkc.iter() {
                write!(file, "{}", char.escape_unicode())?;
            }
            write!(file, "\", \"")?;
            for char in test.nfkd.iter() {
                write!(file, "{}", char.escape_unicode())?;
            }
            writeln!(file, "\"),")?;
        }
        writeln!(file, "]")?;
        Ok(())
    }
}

impl FromStr for NormalizationTests {
    type Err = ();
    fn from_str(str: &str) -> Result<NormalizationTests, ()> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # source
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfc
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfd
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfkc
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfkd
                  \x20
                \#"
            ).unwrap();
        }

        // estimate of number of test cases
        let mut tests = Vec::with_capacity(0x5000);

        for capture in REGEX.captures_iter(str) {
            tests.push(NormalizationTest {
                source: capture[1]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfc: capture[2]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfd: capture[3]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfkc: capture[4]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfkd: capture[5]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            })
        }

        Ok(NormalizationTests(tests.into_boxed_slice()))
    }
}

fn read_normalization_tests() -> io::Result<NormalizationTests> {
    let mut file = File::open(Path::new("data/ucd/test/NormalizationTest.txt"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.parse().expect("Failed to parse NormalizationTest"))
}

pub fn generate<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    println!("> unic::normal::tests::tables::conformance_tests_data");
    read_normalization_tests()?.emit(dir)?;
    Ok(())
}
