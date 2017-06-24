# UNIC Unicode API

This document introduces UNIC's API for Unicode data and algorithm.

See [Unicode and Rust](Unicode_and_Rust.md) for basic Unicode concepts and how they appear in
Rust.


## Unicode Character Properties

Unicode Character Properties are a major part of the Unicode Standard, defined in *Section 3.5,
Properties* of the [Unicode Standard](http://www.unicode.org/versions/latest/) and explained
more in *Unicode Character Database (UCD)* ([UAX\#44](http://www.unicode.org/reports/tr44/)).
Some of these properties are now *deprecated*, meaning that they are no longer recommended for
use, therefore UNIC will not implement these properties unless there is clear demand for them.

Other specifications published by the Unicode Consortium, like the *Unicode IDNA Compatibility
Processing* ([UTS\#46](www.unicode.org/reports/tr46/)) and the *Unicode Emoji*
([UTS\#51](www.unicode.org/reports/tr51/)) also define their own character properties. These
properties are described in respective Unicode Standard Annexes (UAX), Unicode Technical
Standards (UTS), or other Unicode Technical Reports (UTR). Some of these specifications are
*withdrawn*, *suspended*, or *superseded* by other documents, and therefore will not be
implemented by UNIC. See the [Unicode Technical Reports](http://www.unicode.org/reports/) page
for a complete list of these specifications and their current status.


### Naming Convension

The character properties defined in these specifications follow a common naming convension. The
UNIC API for character properties is based on this convension and tries to stay as close as
possible to this naming schemes, making it easier to use the library when familiar with the
Unicode convensions.

Each charcter property and (non-numeric) property value has a name and an abbreviation. For
example:

|                    | *Name*           | *Abbreviation* |
|--------------------|------------------|----------------|
| **Property**       | General_Category | gc             |
| **Property Value** | Uppercase_Letter | Lu             |
| **Property Value** | Cased_Letter     | LC             |


### Unicode Character Database

| **Property Name** | **Property Type** | **UNIC Status** | **UNIC Implementation Type** |
|-------------------|-------------------|-----------------|------------------------------|
| Age               | Catalog           | `unic-ucd-age`  | `enum`                       |
| Bidi_Class        | Enumeration       | `unic-ucd-bidi` | `enum`                       |


## Named Unicode Algorithms

Unicode [defines](http://www.unicode.org/versions/Unicode10.0.0/ch03.pdf) Named
Unicode Algorithms, that are specified in the Unicode Standard or in other
standards published by the Unicode Consortium.

The following table shows their implementation status in UNIC.

| **Name**                                       | **Reference** | **UNIC Status**                  |
|------------------------------------------------|---------------|----------------------------------|
| Canonical Ordering                             | Section 3.11  | `unic-ucd-normal`                |
| Canonical Composition                          | Section 3.11  | `unic-ucd-normal`                |
| Normalization                                  | Section 3.11  | `unic-ucd-normal`                |
| Hangul Syllable Composition                    | Section 3.12  | unic-ucd-normal (Not public yet) |
| Hangul Syllable Decomposition                  | Section 3.12  | unic-ucd-normal (Not public yet) |
| Hangul Syllable Name Generation                | Section 3.12  | Not Implemented                  |
| Default Case Conversion                        | Section 3.13  | Not Implemented                  |
| Default Case Detection                         | Section 3.13  | Not Implemented                  |
| Default Caseless Matching                      | Section 3.13  | Not Implemented                  |
| Bidirectional Algorithm (UBA)                  | UAX \#9       | `unic-bidi`                      |
| Line Breaking Algorithm                        | UAX \#14      | Not Implemented                  |
| Character Segmentation                         | UAX \#29      | Not Implemented                  |
| Word Segmentation                              | UAX \#29      | Not Implemented                  |
| Sentence Segmentation                          | UAX \#29      | Not Implemented                  |
| Hangul Syllable Boundary Determination         | UAX \#29      | Not Implemented                  |
| Standard Compression Scheme for Unicode (SCSU) | UTS \#6       | Not Implemented                  |
| Unicode Collation Algorithm (UCA)              | UTS \#10      | Not Implemented                  |
