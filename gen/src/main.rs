extern crate getopts;
use getopts::Options;
use std::process;
use std::env;

mod utils;
mod ucd;

fn abort_print_usage(opts: Options) -> ! {
    println!(
        "{}",
        opts.usage("Usage: cargo run --package=unic-gen -- MOD [options]")
    );
    process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e),
    };

    if matches.opt_present("h") {
        abort_print_usage(opts);
    }

    if matches.free.is_empty() {
        abort_print_usage(opts);
    }

    match matches.free[0].as_str() {
        "ucd" => ucd::gen_tables(),
        _ => abort_print_usage(opts),
    }
}
