// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate assert_cli;

use assert_cli::Assert;

// At the moment, there's no way to test stdout value for an exact string, therefore a mix of
// `is()`, `contains()`, and `doesnt_contain()` is test trailing newline behavior.
// See <https://github.com/killercup/assert_cli/issues/77> for details and updates.

fn bin() -> Assert {
    Assert::cargo_binary("unic-echo")
}

#[test]
fn test_plain_text_input() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(args)
    }

    // No args
    run(&[])
        .stdout()
        .is("")
        .stdout()
        .contains("\n")
        .stdout()
        .doesnt_contain("\n\n")
        .unwrap();

    // One arg
    run(&["Hello"])
        .stdout()
        .is("Hello")
        .stdout()
        .contains("Hello\n")
        .stdout()
        .doesnt_contain("\nHello")
        .stdout()
        .doesnt_contain("Hello\n\n")
        .unwrap();

    // Two args
    run(&["Hello", "World"])
        .stdout()
        .is("Hello World")
        .stdout()
        .contains("Hello World\n")
        .stdout()
        .doesnt_contain("\nHello")
        .stdout()
        .doesnt_contain("Hello World\n\n")
        .unwrap();

    // Valid UTF-8 args
    run(&["Hello world!", "سلام به همه!"])
        .stdout()
        .is("Hello world! سلام به همه!")
        .unwrap();

    // Invalid UTF-8 args
    // We cannot test invalid UTF-8 args via `assert_cli`.
    // TODO: Find a way to test invalid UTF-8 args.
}

#[test]
fn test_no_newline_output() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(&["--no-newline"]).with_args(args)
    }

    // No args
    run(&[])
        .stdout()
        .is("")
        .stdout()
        .doesnt_contain("\n")
        .unwrap();

    // One arg
    run(&["Hello"])
        .stdout()
        .is("Hello")
        .stdout()
        .doesnt_contain("Hello\n")
        .unwrap();

    // Two args
    run(&["Hello", "World"])
        .stdout()
        .is("Hello World")
        .stdout()
        .doesnt_contain("Hello World\n")
        .unwrap();
}

// == Input Formats ==

#[test]
fn test_codepoints_input() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(&["--input", "codepoints"]).with_args(args)
    }

    // No args
    run(&[]).stdout().is("").unwrap();

    // One arg
    run(&[
        "48 U+65 U+006c U+0006c U+000000000006f", // Hello
    ])
    .stdout()
    .is("Hello")
    .unwrap();

    // Non-ASCII
    run(&[
        "633 0644 000627 0000000000000645", // سلام
    ])
    .stdout()
    .is("سلام")
    .unwrap();
}

#[test]
fn test_utf8_hex_input() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(&["--input", "utf8-hex"]).with_args(args)
    }

    // No args
    run(&[]).stdout().is("").unwrap();

    // One arg
    run(&[
        "48 65 6c 6c 6f", // Hello
    ])
    .stdout()
    .is("Hello")
    .unwrap();

    // Two args
    run(&[
        "48 65 6c 6c 6f",           // Hello
        "20",                       // SPACE
        "0x57 0x6f 0x72 0x6c 0x64", // World
    ])
    .stdout()
    .is("Hello World")
    .unwrap();

    // Missing trailing low digit
    run(&[
        "48 65 6c 6c 6", // Hell_
    ])
    .stdout()
    .is("Hell\u{6}")
    .unwrap();

    // Non-ASCII
    run(&[
        "D8 B3 D9 84 D8 A7 D9 85 0A", // سلام
    ])
    .stdout()
    .is("سلام")
    .stdout()
    .contains("سلام\n")
    .unwrap();
}

#[test]
fn test_utf16_hex_input() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(&["--input", "utf16-hex"]).with_args(args)
    }

    // Non-ASCII
    run(&[
        "0633 0x0644 0627,0645", // سلام
    ])
    .stdout()
    .is("سلام")
    .stdout()
    .contains("سلام\n")
    .unwrap();
}

// == Output Formats ==

#[test]
fn test_codepoints_output() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(&["--output", "codepoints"]).with_args(args)
    }

    // No args
    run(&[]).stdout().is("").unwrap();

    // One arg
    run(&["Hello"])
        .stdout()
        .is("U+0048 U+0065 U+006C U+006C U+006F")
        .unwrap();

    // Non-ASCII
    run(&["سلام"])
        .stdout()
        .is("U+0633 U+0644 U+0627 U+0645")
        .unwrap();

    // Non-BMP
    run(&["\u{1F1FA}\u{1F1F3}\u{1F1EE}\u{1F1E8}\u{1F49F}"])
        .stdout()
        .is("U+1F1FA U+1F1F3 U+1F1EE U+1F1E8 U+1F49F")
        .unwrap();
}

#[test]
fn test_utf8_hex_output() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(&["--output", "utf8-hex"]).with_args(args)
    }

    // No args
    run(&[]).stdout().is("").unwrap();

    // One arg
    run(&["Hello"])
        .stdout()
        .is("0x48 0x65 0x6C 0x6C 0x6F")
        .unwrap();

    // Non-ASCII
    run(&["سلام"])
        .stdout()
        .is("0xD8 0xB3 0xD9 0x84 0xD8 0xA7 0xD9 0x85")
        .unwrap();

    // Non-BMP
    run(&["\u{1F49F}"])
        .stdout()
        .is("0xF0 0x9F 0x92 0x9F")
        .unwrap();
}

#[test]
fn test_utf16_hex_output() {
    fn run(args: &[&str]) -> Assert {
        bin().with_args(&["--output", "utf16-hex"]).with_args(args)
    }

    // No args
    run(&[]).stdout().is("").unwrap();

    // One arg
    run(&["Hello"])
        .stdout()
        .is("0x0048 0x0065 0x006C 0x006C 0x006F")
        .unwrap();

    // Non-ASCII
    run(&["سلام"])
        .stdout()
        .is("0x0633 0x0644 0x0627 0x0645")
        .unwrap();

    // Non-BMP
    run(&["\u{1F1FA}\u{1F1F3}\u{1F1EE}\u{1F1E8}\u{1F49F}"])
        .stdout()
        .is("0xD83C 0xDDFA 0xD83C 0xDDF3 0xD83C 0xDDEE 0xD83C 0xDDE8 0xD83D 0xDC9F")
        .unwrap();
}

#[test]
fn test_rust_escape_output() {
    fn run(args: &[&str]) -> Assert {
        bin()
            .with_args(&["--output", "rust-escape"])
            .with_args(args)
    }

    // No args
    run(&[]).stdout().is("\"\"").unwrap();

    // One arg
    run(&["Hello"]).stdout().is("\"Hello\"").unwrap();

    // Non-ASCII
    run(&["hello", "سلام"])
        .stdout()
        .is("\"hello \\u{633}\\u{644}\\u{627}\\u{645}\"")
        .unwrap();

    // Non-BMP
    run(&["\u{1f1fa}\u{1f1f3}\u{1f1ee}\u{1f1e8}\u{1f49f}"])
        .stdout()
        .is("\"\\u{1f1fa}\\u{1f1f3}\\u{1f1ee}\\u{1f1e8}\\u{1f49f}\"")
        .unwrap();
}
