# UNIC Command-Line Tools

[![Crates.io](https://img.shields.io/crates/v/unic-cli.svg)](https://crates.io/crates/unic-cli)

This package provides command-line tools for developers, helping with common
Unicode tasks, like generating, transforming, and inspecting strings.


## How to Install

Install the `unic-cli` package using [Cargo](https://doc.rust-lang.org/stable/cargo/).

```bash
$ cargo install unic-cli
```


## How to Use It

### Echo Command

The `unic-echo` command generates Unicode strings, similar to the common `echo`
command.

```bash
# Normally, the output ends with a newline
$ unic-echo Hello سلام
Hello سلام

# Get the codepoints or UTF-8/UTF-16, instead of plain text
$ unic-echo Hello سلام -o codepoints
U+0048 U+0065 U+006C U+006C U+006F U+0020 U+0633 U+0644 U+0627 U+0645

# Or see how to escape as a Rust string literal
$ unic-echo Hello سلام -o rust-escape
"Hello \u{633}\u{644}\u{627}\u{645}"

# Also the input can be in codepoints, or UTF-8/UTF-16 hex
$ unic-echo -i codepoints U+0 U+20 U+41 U+A0 -o rust-escape
"\u{0} A\u{a0}"

$ unic-echo --help
Write arguments to the standard output

USAGE:
    unic-echo [FLAGS] [OPTIONS] [STRINGS]...

FLAGS:
    -h, --help          Prints help information
    -n, --no-newline    No trailing newline
    -V, --version       Prints version information

OPTIONS:
    -i, --input <FORMAT>     Specify input format (see list below)
    -o, --output <FORMAT>    Specify output format (see list below)

ARGS:
    <STRINGS>...    Input strings (expected valid Unicode)

INPUT FORMATS:
    plain                   Plain Unicode characters (default)
    codepoints              Unicode codepoints (hex)
    utf8-hex                UTF-8 bytes (hex)
    utf16-hex               UTF-16 words (hex)

OUTPUT FORMATS:
    plain                   Plain Unicode characters (default)
    codepoints              Unicode codepoints (hex)
    utf8-hex                UTF-8 bytes (hex)
    utf16-hex               UTF-16 words (hex)

    braces-escape           String literal with \u{...} escapes for
    | js6-escape            control and non-ASCII characters
    | rust-escape

    braces-escape-all       String literal with \u{...} escapes for
    | js6-escape-all        all characters
    | rust-escape-all

    braces-escape-control   String literal with \u{...} escapes for
    | js6-escape-control    control characters
    | rust-escape-control
```

### Inspector Command

The `unic-inspector` command lists the Unicode characters in the input string,
along with their properties.

```bash
$ unic-inspector Hello سلام
 H | U+0048 | LATIN CAPITAL LETTER H | Uppercase_Letter
 e | U+0065 | LATIN SMALL LETTER E   | Lowercase_Letter
 l | U+006C | LATIN SMALL LETTER L   | Lowercase_Letter
 l | U+006C | LATIN SMALL LETTER L   | Lowercase_Letter
 o | U+006F | LATIN SMALL LETTER O   | Lowercase_Letter
   | U+0020 | SPACE                  | Space_Separator
 س | U+0633 | ARABIC LETTER SEEN     | Other_Letter
 ل | U+0644 | ARABIC LETTER LAM      | Other_Letter
 ا | U+0627 | ARABIC LETTER ALEF     | Other_Letter
 م | U+0645 | ARABIC LETTER MEEM     | Other_Letter
```

Soon, this tool will allow selecting which properties to show, as well as
support grouping characters by Grapheme Clusters, Words, Sentences, etc.
