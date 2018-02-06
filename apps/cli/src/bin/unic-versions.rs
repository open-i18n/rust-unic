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

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! print_pkg_name_desc_version {
    ( $component:tt ) => {
        println!("Component: {}", unic::$component::PKG_DESCRIPTION);
        println!(
            "Package: {}:{}",
            unic::$component::PKG_NAME,
            unic::$component::PKG_VERSION,
        );
    }
}

macro_rules! print_unicode_version {
    ( $component:tt ) => {
        println!(
            "Unicode Version: {}.{}.{}",
            unic::$component::UNICODE_VERSION.major,
            unic::$component::UNICODE_VERSION.minor,
            unic::$component::UNICODE_VERSION.micro,
        );
    }
}

macro_rules! print_emoji_version {
    ( $component:tt ) => {
        println!(
            "Emoji Version: {}.{}.{}",
            unic::$component::EMOJI_VERSION.major,
            unic::$component::EMOJI_VERSION.minor,
            unic::$component::EMOJI_VERSION.micro,
        );
    }
}

macro_rules! print_pkg_only {
    ( $component:tt ) => {
        print_pkg_name_desc_version!($component);
        println!();
    }
}

macro_rules! print_pkg_with_unicode_version {
    ( $component:tt ) => {
        print_pkg_name_desc_version!($component);
        print_unicode_version!($component);
        println!();
    }
}

macro_rules! print_pkg_with_emoji_version {
    ( $component:tt ) => {
        print_pkg_name_desc_version!($component);
        print_emoji_version!($component);
        println!();
    }
}

fn main() {
    println!("UNIC: Unicode and Internationalization Crates for Rust");
    println!("Package: unic:{}", PKG_VERSION);
    println!();

    print_pkg_only!(char);

    print_pkg_with_unicode_version!(ucd);
    print_pkg_with_unicode_version!(bidi);
    print_pkg_with_unicode_version!(normal);
    print_pkg_with_unicode_version!(idna);

    print_pkg_with_emoji_version!(emoji);
}
