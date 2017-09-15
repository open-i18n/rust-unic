// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod shared;

mod age;
mod bidi;
mod category;
mod core;
mod name;
mod normal;

use std::{fs, io};
use std::path::Path;

pub use self::shared::unicode_data::{UnicodeData, UnicodeDataEntry};
pub use self::shared::version::UnicodeVersion;

/// Generate all tables for the UCD component
pub fn generate() -> io::Result<()> {
    println!(">>> Loading UCD Version");
    let ucd_version = shared::version::read_unicode_version()?;
    println!(">>> Loading UCD UnicodeData");
    let unicode_data = shared::unicode_data::read_unicode_data()?;
    println!(">>> Loading UCD DerivedCoreProperties");
    let _derived_core_properties = shared::derived_core_properties::read_derived_core_properties()?;
    println!(">>> Loading UCD PropList");
    let _prop_list = shared::prop_list::read_prop_list()?;

    let path = Path::new("unic/ucd/core/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    core::generate(path, &ucd_version)?;

    let path = Path::new("unic/ucd/age/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    age::generate(path, &ucd_version)?;

    let path = Path::new("unic/ucd/bidi/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    bidi::generate(path, &ucd_version, &unicode_data)?;

    let path = Path::new("unic/ucd/category/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    category::generate(path, &ucd_version, &unicode_data)?;

    let path = Path::new("unic/ucd/name/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    name::generate(path, &ucd_version, &unicode_data)?;

    let path = Path::new("unic/ucd/normal/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    normal::generate(path, &ucd_version, &unicode_data)?;

    Ok(())
}
