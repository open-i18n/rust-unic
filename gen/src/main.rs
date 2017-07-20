extern crate futures;
extern crate getopts;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate tokio_core;

mod download;

use std::env;

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
    });

    let crates = expand_sub_crates(matches.opt_strs("crate"));

    if crates.is_empty() {
        if !matches.opt_present("u") {
            print_usage(&opts);
        }
        return;
    }

    for subcrate in crates {
        println!("Generating sources for crate {}...", subcrate);
        match subcrate {
            _ => println!("No sources to generate for crate {}.", subcrate),
        }
    }
}

fn expand_sub_crates<I, S>(list: I) -> Vec<&'static str>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    list.into_iter()
        .flat_map(|name| match name.as_ref() {
            "NONE" => vec![],
            "unic" => {
                let mut crates =
                    expand_sub_crates(vec!["unic-bidi", "unic-idna", "unic-normal", "unic-ucd"]);
                crates.push("unic");
                crates
            }
            "unic-bidi" => vec!["unic-bidi"],
            "unic-idna" => vec!["unic-idna", "unic-idna-mapping", "unic-idna-punycode"],
            "unic-normal" => vec!["unic-normal"],
            "unic-ucd" => {
                vec![
                    "unic-ucd",
                    "unic-ucd-age",
                    "unic-ucd-bidi",
                    "unic-ucd-category",
                    "unic-ucd-core",
                    "unic-ucd-normal",
                    "unic-ucd-utils",
                ]
            }
            _ => panic!("Unknown crate name {}", name.as_ref()),
        })
        .collect()
}

const TO_INVOKE: &'static str = "cargo run --release --package=unic-gen --";

fn print_usage(opts: &Options) {
    print!("{}", opts.usage(&opts.short_usage(TO_INVOKE)));
}
