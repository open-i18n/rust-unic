extern crate getopts;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

mod download;

use std::env;
use std::collections::HashMap;

use getopts::Options;

fn main() {
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this help menu.");
    opts.optopt(
        "u",
        "unicode-version",
        "The version of the unicode standard to download.  \
         If omitted, the local cache will be used.",
        "VERSION",
    );
    opts.optmulti(
        "",
        "crate",
        "A crate to generate tables for.  \
         This propogates through the tree: if you specify only `unic`, all crates will be built.",
        "CRATE",
    );

    let matches = opts.parse(env::args().skip(1))
        .unwrap_or_else(|e| panic!(e.to_string()));

    if matches.opt_present("h") {
        print_usage(&opts);
        return;
    }

    matches.opt_str("u").map(|version| {
        println!("Downloading Unicode resources version {}...", version);
        download::download(version.as_str())
            .expect("Failed to download Unicode resources");
        println!("Finished.");
    });

    let crates = expand_sub_crates(matches.opt_strs("crate").into_iter());

    if crates.is_empty() {
        if !matches.opt_present("u") {
            print_usage(&opts);
        }
        return;
    }

    println!("Generating sources for {} crates...", crates.len());

    for subcrate in crates {
        println!("Generating sources for crate {}...", subcrate);
        match subcrate {
            _ => println!("No sources to generate for crate {}.", subcrate),
        }
    }
}

const CRATES: &'static str = include_str!("../config/crates.yaml");

fn expand_sub_crates<I>(list: I) -> Vec<String>
where
    I: Iterator<Item = String>,
{
    let crate_mapping: HashMap<String, Vec<String>> =
        serde_yaml::from_str(CRATES)
            .expect("Failed to parse crates.yaml");

    list.into_iter()
        .flat_map(|name| {
            crate_mapping.get(&name)
                .map(|crates| {
                    let mut subcrates = expand_sub_crates(crates.iter().cloned());
                    subcrates.push(name.clone());
                    subcrates
                })
                .unwrap_or_else(|| vec![name])
        })
        .collect()
}

const TO_INVOKE: &'static str = "cargo run --release --package=unic-gen --";

fn print_usage(opts: &Options) {
    print!("{}", opts.usage(&opts.short_usage(TO_INVOKE)));
}
