use super::{ucd_data_dir, ucd_test_data_dir};
use utils::common;

const UCD_URL: &'static str = "http://www.unicode.org/Public/10.0.0/ucd/";

const DATA_FILES: &'static [&'static str] = &[
    "DerivedAge.txt",
    "DerivedNormalizationProps.txt",
    "ReadMe.txt",
    "UnicodeData.txt",
];

const TEST_DATA_FILES: &'static [&'static str] =
    &["BidiCharacterTest.txt", "BidiTest.txt", "NormalizationTest.txt"];

pub fn run() {
    common::cleanup_data(ucd_data_dir()).unwrap();
    common::cleanup_data(ucd_test_data_dir()).unwrap();

    for name in DATA_FILES {
        common::fetch(String::from(UCD_URL) + name, ucd_data_dir());
    }
    for name in TEST_DATA_FILES {
        common::fetch(String::from(UCD_URL) + name, ucd_test_data_dir());
    }
}
