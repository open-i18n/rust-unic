#[macro_use]
extern crate lazy_static;
extern crate getopts;

use getopts::Options;
use std::process;
use std::env;

mod utils;
mod ucd;

fn print_usage(opts: Options) {
    println!(
        "{}",
        opts.usage("Usage: cargo run --package=unic-gen -- [options]")
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "update", "re-download ucd data files");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e),
    };

    if matches.opt_present("h") {
        print_usage(opts);
    } else if matches.opt_present("update") {
        ucd::update();
    } else {
        ucd::generate();
    }
}
