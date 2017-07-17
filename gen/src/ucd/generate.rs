// TODO: Break this file up

use std::path::Path;
use std::fs::File;
use std::io::Read;

use super::ucd_data_dir;
use utils::{common, rustout};

use regex::Regex;

const SCRIPT: &'static str = "gen/src/ucd/generate.rs";

lazy_static! {
    static ref CORE_TABLES: &'static Path = Path::new("components/ucd/core/src/tables");
    static ref AGE_TABLES: &'static Path = Path::new("components/ucd/age/src/tables");
    static ref BIDI_TABLES: &'static Path = Path::new("components/ucd/bidi/src/tables");
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

// == MAIN == //

pub fn run() {
    for &dir in &[*CORE_TABLES, *AGE_TABLES, *BIDI_TABLES, *NORMAL_TABLES] {
        common::cleanup_rsv(dir).unwrap();
    }

    // Core
    emit_unicode_version(*CORE_TABLES);
}
