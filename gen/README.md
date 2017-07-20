# UNIC Code Generation

This is a binary crate to parse Unicode-provided files and to build the `rsv`
tables used in other UNIC packages.

## Usage

We suggest you run via cargo at the workspace root.
We have not tested doing so from a build binary.

```bash
cargo run --release --package=unic-gen -- --help
```

## Notes

This crate should not be published; it is an assistance tool to the development
of UNIC only. The generated files are published as part of their respective crates
such that the end user does not need to download the larger data files and generate
the `rsv` files themselves.

## TODO

- Take the unicode version on `--update`
- Allow only updating data for a single module?
- Use [hyper](https://crates.io/crates/hyper) or similar for fetch instead of curl?
