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

- [X] `core`: Providing basic information about UCD data, specifically the
  [Unicode Version](http://www.unicode.org/versions/).
- [X] `utils`: Providing basic utilities for working with Unicode characters.


- [X] `age`: Character Age.
- [ ] `name`: Character Name.
- [ ] `category`: Character General Category.

- [ ] `block`: Character Block properties
- [ ] `script`: Character Script properties.

- [ ] `normal`: Normalization properties.
- [ ] `case`: Character case properties.
- [ ] `grapheme`: Character grapheme properties.
- [ ] `numeric`: Character numeric properties.

- [X] `bidi`: Bidirectional properties. (Hebrew, Arabic, ...)
- [ ] `joining`: Cursive joining properties. (Arabic, Syriac, ...)
- [ ] `ea-width`: East Asian Width properties.

See <http://unicode.org/reports/tr44/#Property_List_Table> for the complete
list of properties defined in UCD. Eventually, all these properties will be
available by under `unic-ucd`.
