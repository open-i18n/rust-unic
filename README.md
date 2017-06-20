# UNIC: Unicode and Internationalization Crates for Rust

[![UNIC-Logo](.cargo/logo.png)](https://github.com/behnam/rust-unic/)

[![Unicode-10.0.0](https://img.shields.io/badge/unicode-10.0.0-red.svg)](http://www.unicode.org/versions/Unicode10.0.0/)
[![Release](https://img.shields.io/github/release/behnam/rust-unic.svg)](https://github.com/behnam/rust-unic/)
[![Travis](https://img.shields.io/travis/behnam/rust-unic.svg)](https://travis-ci.org/behnam/rust-unic/)
[![Crates.io](https://img.shields.io/crates/v/unic.svg)](https://crates.io/crates/unic/)
[![Documentation](https://docs.rs/unic/badge.svg)](https://docs.rs/unic/)

https://github.com/behnam/rust-unic

**UNIC** is a project to develop components for the Rust programming language
to provide high-quality and easy-to-use crates for Unicode
and Internationalization data and algorithms. In other words, it's like
[ICU](http://site.icu-project.org/) for Rust, written completely in Rust, mostly
in *safe* mode, but also benifiting from performance gains of *unsafe* mode when
possible.

## Project Goal

The goal for UNIC is to provide access to all levels of  Unicode and
Internationalization functionalities, starting from Unicode character
properties, to Unicode algorithms for processing text, and more advanced
(locale-based) processes based on Unicode Common Locale Data Repository (CLDR).

Other standards and best practices, like IETF RFCs, are also implemented, as
needed by Unicode/CLDR components, or common demand.

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

## Components and Their Organization

UNIC *Components* have a tree organization, starting from the
[`components`](components/) directory, containing the *major components*. Each
major component, in turn, may host *minor component*.

API of major components are designed for the end-users of the libraries, and
are expected to be extensively documented and accompanies with code examples.

In contrast to major components, minor components act as providers of data and
algorithms for the higher-level, and their API is expected to be more
performing, and possibly providing multiple ways of accessing the data.

### The UNIC Super-Crate

The [`unic`](https://crates.io/crates/unic) super-crate (this crate) is a
collection of all UNIC (major) components, providing an easy way of access to
all functionalities, when all or many are needed, instead of importing
components one-by-one. This crate ensures all components imported are
compatible in algorithms and consistent data-wise.

Main code examples and cross-component integration tests are implemented under
this crate.

### Major Components

-   [`unic::ucd`](components/ucd): Unicode Character Database.

-   [`unic::bidi`](components/bidi): Unicode Bidirectional Algorithm (USA\#9).

-   [`unic::normal`](components/normal): Unicode Normalization Forms (USA\#15).

-   [`unic::idna`](components/idna): Unicode IDNA Compatibility Processing
    (UTS\#46).

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

## Versioning

Being a Rust repository, we follow *Semantic Versioning* practices, meaning
that API remains stable for `0.0.x` and `Y.x` ($$where Y > 0$$). So, any
API-breaking change gets a *big* version bump.

Besides API-breaking changes, big version bumps will also happen for every
major or minor Unicode version update. (As of mid-2017, Unicode is expected to
have a major update every year, with no minor updates.) See [Unicode Character
Encoding Stability Policies](http://unicode.org/policies/stability_policy.html)
for details on Unicode versioning and other stability policies.

For small version bumps, all parents of components getting an update will also
get a version bump. This ensures that depending on any super-crate (`unic`)
version requires all the components to be up-to-date.

When updating package versions, all components being updated get the same
version of the super-crate. Therefore, if a component A was at version `0.2.4`,
and now `unic` package is bumping to version `0.2.7`, component A will also
become `0.2.7`. This method works very well with Semantic Versioning and allows
easier understand of relations between different versions of components, to the
super-crate and among themselves.
