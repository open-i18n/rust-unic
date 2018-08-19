# Contributing to UNIC

Thank you for your interest in contributing to UNIC! There are many ways to
contribute, and we appreciate all of them.

If you have questions, please ask on the [Gitter
room](https://gitter.im/open-i18n/rust-unic) or join the [IRC channel
\#unic](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23unic).

## Feature Requests and Bug Reports

To request for new components or new features for the existing components, or to
report a bug, please [file an issue on
GitHub](https://github.com/open-i18n/rust-unic/issues/new).

## Code Organization

You find these directories in the project root:

-   [`/data/`](data/): source data; used to generate data tables for consumption
    in the code; or, used at runtime by integration tests (like conformance
    tests) and benches.

-   [`/docs/`](docs/): project high-level documentations, such as tutorials and
    guidelines.

-   [`/etc/`](etc/): scripts for project maintenance and deployment.

-   [`/tools/`](tools/): tooling for generating data tables from source data.

-   [`/unic/`](unic/): source code for the `unic` super-crate package. Also,
    under this directory goes all major UNIC components, each under their own
    directory. The components are separate by multiple aspects: source of
    data/algorithm (like Unicode, CLDR, IETF), abstraction level (like
    character-level functionalities vs. string-level functionalities), and
    practicality for users of the library.

## Submitting a Pull Request

Please consider running through these steps before submitting a PR for UNIC:

1.  If you're adding a new component, add it to the `COMPONENTS` list in
    [`/etc/common.sh`](etc/common.sh), somewhere after all its dependencies.
    (This list is used for packaging and publishing the components, as `cargo
    package` does not support workspaces, yet.

2.  Tests that depend on large data, whether directly reading from `/data/`
    directory, or using data tables generated from source data, should be added
    as integration tests, in `tests/` directory under the relevant component. If
    it's a benchmarking test, it should go under `benches/`.

3.  If you're adding new integration tests or benches that read directly from
    the `/data/` directory, it should be excluded from packaging, as it won't be
    able to access the data when unpacked. Add each such test file to the
    `exclude` list in the Cargo manifest file (`Cargo.toml`).

4.  For each component, referencing to paths outside of the component directory
    should be limited to these specific cases:

    -   In `[dependencies]` section of the Cargo manifest, setting `path` for
        other UNIC components.

    -   In integration tests and benches, reading from source data.

5.  For each component, we keep the source code organized as this:

    -   The library file (`src/lib.rs`) contains version information of the
        package (like `CARGO_PKG_VERSION`) and the data (like
        `UNICODE_VERSION`), and API is `pub use`d from other crates and local
        modules.

    -   New types go into their own modules, and kept separate based on
        abstraction levels in the specifications and implementation. For
        example, the `BidiClass` type, representing the Unicode *Bidi_Class*
        character property, goes into its own `bidi_class` module, which also
        contains other types directly related to the definition, like
        `BidiClassCategory`.

    -   Traits for non-UNIC types, like the Rust core `char` and `str` types to
        into separate modules, usually named `trait.rs`.

6.  Format the code (Rust and Python) using the
    [`/etc/format.sh`](etc/format.sh). This allows everyone to use
    auto-formatting and not worry about manual code formatting.  Please make use
    you are using the latest version of
    [`rustfmt-nightly`](https://crates.io/crates/rustfmt-nightly) before running
    the script.

    If you have suggestions to change the Rust formatting style, please
    submitting a separate PR, updating [`/.rustfmt.toml`](.rustfmt.toml) in one
    commit, and applying the changes in another diff.

7.  For pre-`1.0.0` development, we try to have a new release after each new
    Rust release. Therefore, there's no need to make new releases after each
    major or minor component, and as a result, no reason to bump version in PRs.

## Miscellaneous

### TODO/FIXME Tags

Use the `FIXME` comment tag to mark something that's broken. The only data after
a `FIXME` tag would be the detail of what's broken.

Use the `TODO` comment tag to describe what needs to be done in the future. You
can also mark different kinds of `TODO` items, as shown below.

If no metadata, leave the tag empty.

```rust
// TODO: Add more tests for this and that case.
```

Mark GitHub issues/pulls with the `GH-` prefix, which is more portable/readable
that the previous `#nnn` format.

```rust
// TODO(GH-205): Write more on FIXME/TODO tags.
```

For work depending or waiting on updates to Rust, mark them with:
-   `MIN_RUST_VERSION`, if the feature is already stable;
-   `FUTURE_RUST`, if the feature is yet unstable (nightly-only).

```rust
// TODO(MIN_RUST_VERSION): Drop this after Rust 1.23.0.
...

// TODO(FUTURE_RUST): Replace with char::MAX_UTF*_LEN when available.
...
```

If no other context/metadata, but a specific person is expected to follow-up,
add their username.

```rust
// TODO(behnam): Write more on FIXME/TODO tags.
```

