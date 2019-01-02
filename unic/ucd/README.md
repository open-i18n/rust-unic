# UNIC — Unicode Character Database

[![Crates.io](https://img.shields.io/crates/v/unic-ucd.svg)](https://crates.io/crates/unic-ucd)
[![Documentation](https://docs.rs/unic-ucd/badge.svg)](https://docs.rs/unic-ucd/)

This UNIC component provides access to character properties as defined in the
[Unicode® Standard Annex #44 - Unicode Character
Database](http://unicode.org/reports/tr44/).

UCD is a UNIC super-crate, composed of smaller crates that provide data in
specific areas, therefore, allowing access only to the data needed instead of
forcing dependent crates to import all UCD data.

## Crates
Here's a list of components (available or planned) for this super-crate:

- [X] `version`: The [Unicode Version](https://www.unicode.org/versions/) of UCD
  data.
- [X] `common`: Common properties, such as Alphabetic, White-Space, Control and
  Numeric.

- [X] `age`: Age property.
- [X] `bidi`: Bidirectional properties. (Hebrew, Arabic, ...)
- [X] `block`: Block properties.
- [X] `case`: Letter Case properties.
- [X] `category`: General_Category property.
- [X] `hangul`: Hangul Syllable Composition & Decomposition.
- [X] `ident`: Identifier properties.
- [X] `name`: Name property.
- [X] `normal`: Normalization properties.
- [X] `segment`: Segmentation properties.

- [ ] `ea-width`: East Asian Width properties.
- [ ] `joining`: Cursive joining properties. (Arabic, Syriac, ...)
- [ ] `numeric`: Other character numeric properties.
- [ ] `script`: Script properties.

See <http://unicode.org/reports/tr44/#Property_List_Table> for the complete
list of properties defined in UCD. Eventually, all these properties will be
available by under `unic-ucd`.
