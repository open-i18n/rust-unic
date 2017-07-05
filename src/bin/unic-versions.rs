// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code)]

//! Command-line tool to list versions of UNIC components.


extern crate unic;


const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    println!("# UNIC - Unicode and Internationalization Crates");
    println!("Package Version: {}", PKG_VERSION);
    println!("");

    println!("# Component: {}", unic::ucd::PKG_NAME);
    println!("{}", unic::ucd::PKG_DESCRIPTION);
    println!("Package Version: {}", PKG_VERSION);
    println!(
        "Unicode Version: {}.{}.{}",
        unic::ucd::UNICODE_VERSION.major,
        unic::ucd::UNICODE_VERSION.minor,
        unic::ucd::UNICODE_VERSION.micro,
    );
    println!("");

    println!("# Component: {}", unic::bidi::PKG_NAME);
    println!("{}", unic::bidi::PKG_DESCRIPTION);
    println!("Package Version: {}", PKG_VERSION);
    println!(
        "Unicode Version: {}.{}.{}",
        unic::bidi::UNICODE_VERSION.major,
        unic::bidi::UNICODE_VERSION.minor,
        unic::bidi::UNICODE_VERSION.micro,
    );
    println!("");

    println!("# Component: {}", unic::idna::PKG_NAME);
    println!("{}", unic::idna::PKG_DESCRIPTION);
    println!("Package Version: {}", PKG_VERSION);
    println!(
        "Unicode Version: {}.{}.{}",
        unic::idna::UNICODE_VERSION.major,
        unic::idna::UNICODE_VERSION.minor,
        unic::idna::UNICODE_VERSION.micro,
    );
    println!("");

    println!("# Component: {}", unic::normal::PKG_NAME);
    println!("{}", unic::normal::PKG_DESCRIPTION);
    println!("Package Version: {}", PKG_VERSION);
    println!(
        "Unicode Version: {}.{}.{}",
        unic::normal::UNICODE_VERSION.major,
        unic::normal::UNICODE_VERSION.minor,
        unic::normal::UNICODE_VERSION.micro,
    );
    println!("");
}
