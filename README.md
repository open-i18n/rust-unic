# UNIC: Unicode and Internationalization Crates for Rust

[![UNIC-logo](docs/images/UNIC-logo.png)](https://github.com/behnam/rust-unic/)

[![Travis](https://img.shields.io/travis/behnam/rust-unic/master.svg?label=Linux%20build)](https://travis-ci.org/behnam/rust-unic/)
[![AppVeyor](https://img.shields.io/appveyor/ci/behnam/rust-unic/master.svg?label=Windows%20build)](https://ci.appveyor.com/project/behnam/rust-unic)
[![Unicode-10.0.0](https://img.shields.io/badge/unicode-10.0.0-red.svg)](http://www.unicode.org/versions/Unicode10.0.0/)
[![Release](https://img.shields.io/github/release/behnam/rust-unic.svg)](https://github.com/behnam/rust-unic/)
[![Crates.io](https://img.shields.io/crates/v/unic.svg)](https://crates.io/crates/unic/)
[![Documentation](https://docs.rs/unic/badge.svg)](https://docs.rs/unic/)
[![Gitter](https://img.shields.io/gitter/room/behnam/rust-unic.svg)](https://gitter.im/behnam/rust-unic)
[![IRC](https://img.shields.io/badge/chat-on%20irc-brightgreen.svg)](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23unic)

<https://github.com/behnam/rust-unic>

**UNIC** is a project to develop components for the Rust programming language
to provide high-quality and easy-to-use crates for Unicode
and Internationalization data and algorithms. In other words, it's like
[ICU](http://site.icu-project.org/) for Rust, written completely in Rust, mostly
in *safe* mode, but also benifiting from performance gains of *unsafe* mode when
possible.

## Project Goal

The goal for UNIC is to provide access to all levels of Unicode and
Internationalization functionalities, starting from Unicode character
properties, to Unicode algorithms for processing text, and more advanced
(locale-based) processes based on Unicode Common Locale Data Repository (CLDR).

Other standards and best practices, like IETF RFCs, are also implemented, as
needed by Unicode/CLDR components, or common demand.

## Project Status

At the moment, in mid-2017, UNIC is under heavy development: the API is updated
frequently on `master` branch, and there will be API breakage between each `0.x`
release. Please see [open issues](https://github.com/behnam/rust-unic/issues)
for changes planed.

We expect to have the `1.0` version released in late-2017/early-2018 and
maintain a stable API afterwards, with possibly one or two API udpates per year
for the first couple of years.

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
[`unic`](unic) root, containing the *major components*. Each major component, in
turn, may host one or more *minor components*.

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

-   [`unic::ucd`](unic/ucd): Unicode Character Database.
    [![Crates.io](https://img.shields.io/crates/v/unic-ucd.svg)](https://crates.io/crates/unic-ucd/)

-   [`unic::bidi`](unic/bidi): Unicode Bidirectional Algorithm (USA\#9).
    [![Crates.io](https://img.shields.io/crates/v/unic-bidi.svg)](https://crates.io/crates/unic-bidi/)

-   [`unic::normal`](unic/normal): Unicode Normalization Forms (USA\#15).
    [![Crates.io](https://img.shields.io/crates/v/unic-normal.svg)](https://crates.io/crates/unic-normal/)

-   [`unic::idna`](unic/idna): Unicode IDNA Compatibility Processing
    (UTS\#46).
    [![Crates.io](https://img.shields.io/crates/v/unic-idna.svg)](https://crates.io/crates/unic-idna/)

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

*   **Shared bootstraping.** Considerable amount of extending Unicode/i18n
    functionalities depends on converting source Unicode/locale data into
    structured formats for the destination programming language. In a combined
    repository, it's easier to maining these bootstraping tools, expand
    coverage, and use better data structures for more efficiency.

## Documentation

* [Unicode and Rust](docs/Unicode_and_Rust.md)
* [UNIC Versioning](docs/Versioning.md)
* [UNIC Unicode API](docs/Unicode_API.md)
* [UNIC API Guideline](docs/API_Guideline.md)
* [UNIC API Reference](https://docs.rs/unic/) (autogenerated on docs.rs)


## How to Use UNIC

In `Cargo.toml`:

```toml
[dependencies]
unic = "0.5.0"  # This has Unicode 10.0.0 data and algorithms
```

And in `main.rs`:

```rust
extern crate unic;

use unic::bidi::BidiInfo;
use unic::normal::StrNormalForm;
use unic::ucd::{Age, BidiClass, CharAge, CharBidiClass, StrBidiClass, UnicodeVersion};
use unic::ucd::normal::compose;

fn main() {

    // Age

    assert_eq!(Age::of('A'), Age::Assigned(UnicodeVersion { major: 1, minor: 1, micro: 0 }));
    assert_eq!(Age::of('\u{A0000}'), Age::Unassigned);
    assert_eq!(
        Age::of('\u{10FFFF}'),
        Age::Assigned(UnicodeVersion { major: 2, minor: 0, micro: 0 })
    );

    if let Some(uni_ver) = '🦊'.age().assigned() {
        assert_eq!(uni_ver.major, 9);
        assert_eq!(uni_ver.minor, 0);
        assert_eq!(uni_ver.micro, 0);
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

    let bidi_info = BidiInfo::new(&text, None);
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

    // Normalization

    assert_eq!(compose('A', '\u{30a}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");

}
```

You can find more examples under [`examples`](examples/) and [`tests`](tests/)
directories. (And more to come, soon...)

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
