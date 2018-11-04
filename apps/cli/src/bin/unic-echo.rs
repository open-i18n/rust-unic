// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clap;

#[macro_use]
extern crate unic_cli;

use std::io::{self, Write};

use clap::{Arg, ErrorKind};

use unic_cli::{parsers, writers, Result};

unic_arg_enum!{
    #[derive(Debug)]
    enum InputFormat {
        Plain,
        // Unicode + UTF
        Codepoint,
        Codepoints,
        Utf8Hex,
        Utf16Hex
    }
}

macro_rules! input_formats_help {
    () => {
        "INPUT FORMATS:
    plain                   [default] Plain Unicode characters
    codepoints              Unicode codepoints (hex)
    utf8-hex                UTF-8 bytes (hex)
    utf16-hex               UTF-16 words (hex)
"
    };
}

unic_arg_enum!{
    #[derive(Debug)]
    enum OutputFormat {
        Plain,
        // Unicode + UTF
        Codepoint,
        Codepoints,
        Utf8Hex,
        Utf16Hex,
        // Braces Escapes
        BracesEscape,
        Js6Escape,
        RustEscape,
        // Braces Escape All
        BracesEscapeAll,
        Js6EscapeAll,
        RustEscapeAll,

        // Braces Escape Control
        BracesEscapeControl,
        Js6EscapeControl,
        RustEscapeControl
    }
}

macro_rules! output_formats_help {
    () => {
        "OUTPUT FORMATS:
    plain                   [default] Plain Unicode characters
    codepoints              Unicode codepoints (hex)
    utf8-hex                UTF-8 bytes (hex)
    utf16-hex               UTF-16 words (hex)

    braces-escape           String literal with \\u{...} escapes for
    | js6-escape            control and non-ASCII characters
    | rust-escape

    braces-escape-all       String literal with \\u{...} escapes for
    | js6-escape-all        all characters
    | rust-escape-all

    braces-escape-control   String literal with \\u{...} escapes for
    | js6-escape-control    control characters
    | rust-escape-control
"
    };
}

fn main() {
    run().expect("IO Error");
}

fn run() -> Result<()> {
    let app = app_from_crate!()
        .about(concat!(
            env!("CARGO_PKG_DESCRIPTION"),
            "\n\n",
            "Write arguments to the standard output",
        ))
        .arg(
            Arg::with_name("no_newline")
                .short("n")
                .long("no-newline")
                .help("No trailing newline"),
        )
        .arg(
            Arg::with_name("STRINGS")
                .multiple(true)
                .help("Input strings (expected valid Unicode)"),
        )
        .arg(
            Arg::with_name("input_format")
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("FORMAT")
                .help("Specify input format (see list below)"),
        )
        .arg(
            Arg::with_name("output_format")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FORMAT")
                .help("Specify output format (see list below)"),
        )
        .after_help(concat!(input_formats_help!(), "\n", output_formats_help!()));
    let matches = app.get_matches();

    // == Read input ==
    let input: String = matches
        .values_of("STRINGS")
        .unwrap_or_default()
        .collect::<Vec<&str>>()
        .join(" ");

    let input_format =
        value_t!(matches, "input_format", InputFormat).unwrap_or_else(|err| match err.kind {
            ErrorKind::ValueValidation => {
                eprintln!("{}", matches.usage());
                err.exit();
            }
            _ => InputFormat::Plain,
        });

    let string: String = match input_format {
        InputFormat::Plain => input,
        InputFormat::Codepoint | InputFormat::Codepoints => parsers::codepoints(&input),
        InputFormat::Utf8Hex => parsers::utf8_hex(&input),
        InputFormat::Utf16Hex => parsers::utf16_hex(&input),
    };
    let chars = string.chars();

    // == Write output ==
    let mut output = io::stdout();

    let output_format =
        value_t!(matches, "output_format", OutputFormat).unwrap_or_else(|err| match err.kind {
            ErrorKind::ValueValidation => err.exit(),
            _ => OutputFormat::Plain,
        });

    match output_format {
        OutputFormat::Plain => write!(output, "{}", string)?,

        // Unicode + UTF
        OutputFormat::Codepoint | OutputFormat::Codepoints => {
            writers::write_as_codepoints(&mut output, chars)?
        }
        OutputFormat::Utf8Hex => writers::write_as_utf8_hex(&mut output, chars)?,
        OutputFormat::Utf16Hex => writers::write_as_utf16_hex(&mut output, chars)?,

        // Escapes
        OutputFormat::BracesEscape | OutputFormat::Js6Escape | OutputFormat::RustEscape => {
            writers::write_with_control_n_unicode_braces_escape(&mut output, chars)?
        }

        OutputFormat::BracesEscapeAll
        | OutputFormat::Js6EscapeAll
        | OutputFormat::RustEscapeAll => writers::write_with_all_braces_escape(&mut output, chars)?,

        OutputFormat::BracesEscapeControl
        | OutputFormat::Js6EscapeControl
        | OutputFormat::RustEscapeControl => {
            writers::write_with_control_braces_escape(&mut output, chars)?
        }
    };

    if !matches.is_present("no_newline") {
        writeln!(output)?;
    }

    Ok(())
}
