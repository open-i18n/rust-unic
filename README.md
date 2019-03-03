# UNIC: Unicode and Internationalization Crates for Rust

[![UNIC-logo](docs/images/UNIC-logo.png)](https://github.com/open-i18n/rust-unic/)

[![Travis](https://img.shields.io/travis/open-i18n/rust-unic/master.svg?label=Linux%20build)](https://travis-ci.org/open-i18n/rust-unic/)
[![AppVeyor](https://img.shields.io/appveyor/ci/open-i18n/rust-unic/master.svg?label=Windows%20build)](https://ci.appveyor.com/project/open-i18n/rust-unic)
[![Rust-1.31.0+](https://img.shields.io/badge/rustc-1.31+-red.svg#MIN_RUST_VERSION)](https://www.rust-lang.org/)
[![Unicode-10.0.0](https://img.shields.io/badge/unicode-10.0.0-red.svg)](https://www.unicode.org/versions/Unicode10.0.0/)
[![Release](https://img.shields.io/github/release/open-i18n/rust-unic.svg)](https://github.com/open-i18n/rust-unic/)
[![Crates.io](https://img.shields.io/crates/v/unic.svg)](https://crates.io/crates/unic/)
[![Documentation](https://docs.rs/unic/badge.svg)](https://docs.rs/unic/)
[![Gitter](https://img.shields.io/gitter/room/open-i18n/rust-unic.svg)](https://gitter.im/open-i18n/rust-unic)

<https://github.com/open-i18n/rust-unic>

**UNIC** is a project to develop components for the Rust programming language
to provide high-quality and easy-to-use crates for Unicode
and Internationalization data and algorithms. In other words, it's like
[ICU](http://site.icu-project.org/) for Rust, written completely in Rust, mostly
in *safe* mode, but also benefiting from performance gains of *unsafe* mode when
possible.

See [UNIC Changelog](CHANGELOG.md) for latest release details.


## Project Goal

The goal for UNIC is to provide access to all levels of Unicode and
Internationalization functionalities, starting from Unicode character
properties, to Unicode algorithms for processing text, and more advanced
(locale-based) processes based on Unicode Common Locale Data Repository (CLDR).

Other standards and best practices, like IETF RFCs, are also implemented, as
needed by Unicode/CLDR components, or common demand.


## Project Status

At the moment UNIC is under heavy development: the API is updated frequently on
`master` branch, and there will be API breakage between each `0.x` release.
Please see [open issues](https://github.com/open-i18n/rust-unic/issues) for changes
planed.

We expect to have the `1.0` version released in 2018 and maintain a stable API
afterwards, with possibly one or two API updates per year for the first couple
of years.


## Design Goals

1.  Primary goal of UNIC is to provide reliable functionality by way of
    easy-to-use API. Therefore, new components are added may not be
    well-optimized for performance, but will have enough tests to show
    conformance to the standard, and examples to show users how they can be
    used to address common needs.

2.  Next major goal for UNIC components is performance and low binary and memory
    footprints. Specially, optimizing runtime for ASCII and other common cases
    will encourage adaptation without fear of slowing down regular development
    processes.

3.  Components are guaranteed, to the extend possible, to provide consistent
    data and algorithms. Cross-component tests are used to catch any
    inconsistency between implementations, without slowing down development
    processes.


## Components and their Organization

UNIC *Components* have a hierarchical organization, starting from the
[`unic`](unic/) root, containing the *major components*. Each major component, in
turn, may host some *minor components*.

API of major components are designed for the end-users of the libraries, and
are expected to be extensively documented and accompanies with code examples.

In contrast to major components, minor components act as providers of data and
algorithms for the higher-level, and their API is expected to be more
performing, and possibly providing multiple ways of accessing the data.

### The UNIC Super-Crate

The [`unic`](https://crates.io/crates/unic) super-crate is a collection of all
UNIC (major) components, providing an easy way of access to all functionalities,
when all or many are needed, instead of importing components one-by-one. This
crate ensures all components imported are compatible in algorithms and
consistent data-wise.

Main code examples and cross-component integration tests are implemented under
this crate.

### Major Components

-   [`unic-char`](unic/char/): Unicode Character Tools.
    [![Crates.io](https://img.shields.io/crates/v/unic-char.svg)](https://crates.io/crates/unic-char/)

-   [`unic-ucd`](unic/ucd/): Unicode Character Database
    ([UAX\#44](https://unicode.org/reports/tr44/)).
    [![Crates.io](https://img.shields.io/crates/v/unic-ucd.svg)](https://crates.io/crates/unic-ucd/)

-   [`unic-bidi`](unic/bidi/): Unicode Bidirectional Algorithm
    ([UAX\#9](https://unicode.org/reports/tr9/)).
    [![Crates.io](https://img.shields.io/crates/v/unic-bidi.svg)](https://crates.io/crates/unic-bidi/)

-   [`unic-normal`](unic/normal/): Unicode Normalization Forms
    ([UAX\#15](https://unicode.org/reports/tr15/)).
    [![Crates.io](https://img.shields.io/crates/v/unic-normal.svg)](https://crates.io/crates/unic-normal/)

-   [`unic-segment`](unic/segment/): Unicode Text Segmentation Algorithms
    ([UAX\#29](https://unicode.org/reports/tr29/)).
    [![Crates.io](https://img.shields.io/crates/v/unic-segment.svg)](https://crates.io/crates/unic-segment/)

-   [`unic-idna`](unic/idna/): Unicode IDNA Compatibility Processing
    ([UTS\#46](https://unicode.org/reports/tr46/)).
    [![Crates.io](https://img.shields.io/crates/v/unic-idna.svg)](https://crates.io/crates/unic-idna/)

-   [`unic-emoji`](unic/emoji/): Unicode Emoji
    ([UTS\#51](https://unicode.org/reports/tr51/)).
    [![Crates.io](https://img.shields.io/crates/v/unic-emoji.svg)](https://crates.io/crates/unic-emoji/)

### Applications

-   [`unic-cli`](apps/cli): UNIC Command-Line Tools
    [![Crates.io](https://img.shields.io/crates/v/unic-cli.svg)](https://crates.io/crates/unic-cli/)


## Code Organization: Combined Repository

Some of the reasons to have a combined repository these components are:

*   **Faster development**. Implementing new Unicode/i18n components very often
    depends on other (lower level) components, which in turn may need
    adjustments—expose new API, fix bugs, etc—that can be developed, tested and
    reviewed in less cycles and shorter times.

*   **Implementation Integrity**. Multiple dependencies on other components
    mean that the components need to, to some level, agree with each other.
    Many Unicode algorithms, composed from smaller ones, assume that all parts
    of the algorithm is using the same version of Unicode data. Violation of
    this assumption can cause inconsistencies and hard-to-catch bugs. In a
    combined repository, it's possible to reach a better integrity during
    development, as well as with cross-component (integration) tests.

*   **Pay for what you need.** Small components (basic crates), which
    cross-depend only on what they need, allow users to only bring in what they
    consume in their project.

*   **Shared bootstrapping.** Considerable amount of extending Unicode/i18n
    functionalities depends on converting source Unicode/locale data into
    structured formats for the destination programming language. In a combined
    repository, it's easier to maintain these bootstrapping tools, expand
    coverage, and use better data structures for more efficiency.


## Documentation

* [Unicode and Rust](docs/Unicode_and_Rust.md)
* [UNIC Versioning](docs/Versioning.md)
* [UNIC Unicode API](docs/Unicode_API.md)
* [UNIC API Guideline](docs/API_Guideline.md)
* [UNIC API Reference](https://docs.rs/unic/) (autogenerated on *docs.rs*)


## How to Use UNIC

In `Cargo.toml`:

```toml
[dependencies]
unic = "0.9.0"  # This has Unicode 10.0.0 data and algorithms
```

And in `main.rs`:

```rust
extern crate unic;

use unic::ucd::common::is_alphanumeric;
use unic::bidi::BidiInfo;
use unic::normal::StrNormalForm;
use unic::segment::{GraphemeIndices, Graphemes, WordBoundIndices, WordBounds, Words};
use unic::ucd::normal::compose;
use unic::ucd::{is_cased, Age, BidiClass, CharAge, CharBidiClass, StrBidiClass, UnicodeVersion};

fn main() {

    // Age

    assert_eq!(Age::of('A').unwrap().actual(), UnicodeVersion { major: 1, minor: 1, micro: 0 });
    assert_eq!(Age::of('\u{A0000}'), None);
    assert_eq!(
        Age::of('\u{10FFFF}').unwrap().actual(),
        UnicodeVersion { major: 2, minor: 0, micro: 0 }
    );

    if let Some(age) = '🦊'.age() {
        assert_eq!(age.actual().major, 9);
        assert_eq!(age.actual().minor, 0);
        assert_eq!(age.actual().micro, 0);
    }

    // Bidi

    let text = concat![
        "א",
        "ב",
        "ג",
        "a",
        "b",
        "c",
    ];

    assert!(!text.has_bidi_explicit());
    assert!(text.has_rtl());
    assert!(text.has_ltr());

    assert_eq!(text.chars().nth(0).unwrap().bidi_class(), BidiClass::RightToLeft);
    assert!(!text.chars().nth(0).unwrap().is_ltr());
    assert!(text.chars().nth(0).unwrap().is_rtl());

    assert_eq!(text.chars().nth(3).unwrap().bidi_class(), BidiClass::LeftToRight);
    assert!(text.chars().nth(3).unwrap().is_ltr());
    assert!(!text.chars().nth(3).unwrap().is_rtl());

    let bidi_info = BidiInfo::new(text, None);
    assert_eq!(bidi_info.paragraphs.len(), 1);

    let para = &bidi_info.paragraphs[0];
    assert_eq!(para.level.number(), 1);
    assert_eq!(para.level.is_rtl(), true);

    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);
    assert_eq!(
        display,
        concat![
            "a",
            "b",
            "c",
            "ג",
            "ב",
            "א",
        ]
    );

    // Case

    assert_eq!(is_cased('A'), true);
    assert_eq!(is_cased('א'), false);

    // Normalization

    assert_eq!(compose('A', '\u{030A}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");

    // Segmentation

    assert_eq!(
        Graphemes::new("a\u{310}e\u{301}o\u{308}\u{332}").collect::<Vec<&str>>(),
        &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"]
    );

    assert_eq!(
        Graphemes::new("a\r\nb🇺🇳🇮🇨").collect::<Vec<&str>>(),
        &["a", "\r\n", "b", "🇺🇳", "🇮🇨"]
    );

    assert_eq!(
        GraphemeIndices::new("a̐éö̲\r\n").collect::<Vec<(usize, &str)>>(),
        &[(0, "a̐"), (3, "é"), (6, "ö̲"), (11, "\r\n")]
    );

    assert_eq!(
        Words::new(
            "The quick (\"brown\") fox can't jump 32.3 feet, right?",
            |s: &&str| s.chars().any(is_alphanumeric),
        ).collect::<Vec<&str>>(),
        &["The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right"]
    );

    assert_eq!(
        WordBounds::new("The quick (\"brown\")  fox").collect::<Vec<&str>>(),
        &["The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", " ", "fox"]
    );

    assert_eq!(
        WordBoundIndices::new("Brr, it's 29.3°F!").collect::<Vec<(usize, &str)>>(),
        &[
            (0, "Brr"),
            (3, ","),
            (4, " "),
            (5, "it's"),
            (9, " "),
            (10, "29.3"),
            (14, "°"),
            (16, "F"),
            (17, "!")
        ]
    );
}
```

You can find more examples under [`examples`](examples/) and [`tests`](tests/)
directories. (And more to be added as UNIC expands...)


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


## Code of Conduct

UNIC project follows **The Rust Code of Conduct**. You can find a copy of it in
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) or online at
<https://www.rust-lang.org/conduct.html>.
