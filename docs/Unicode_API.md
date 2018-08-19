# UNIC Unicode API

This document introduces UNIC's API for Unicode data and algorithm.

See [Unicode and Rust](Unicode_and_Rust.md) for basic Unicode concepts and how they appear in
Rust.

See [UNIC API Checklist](API_Guideline.md) for common Rust API guidelines that UNIC tries to
follow.

## Unicode Character Properties

Unicode Character Properties are a major part of the Unicode Standard, defined in *Section 3.5,
Properties* of the [Unicode Standard](http://www.unicode.org/versions/latest/) and explained
more in *Unicode Character Database (UCD)* ([UAX\#44](http://www.unicode.org/reports/tr44/)).

Some of these properties are now *deprecated*, meaning that they are no longer recommended for
use. Some other properties are considered *contributory* properties. Neither of these groups of
properties will be supported in UNIC, unless there is clear demand for them.

Other specifications published by the Unicode Consortium, like the *Unicode IDNA Compatibility
Processing* ([UTS\#46](www.unicode.org/reports/tr46/)) and the *Unicode Emoji*
([UTS\#51](www.unicode.org/reports/tr51/)) also define their own character properties. These
properties are described in respective Unicode Standard Annexes (UAX), Unicode Technical
Standards (UTS), or other Unicode Technical Reports (UTR). Some of these specifications are
*withdrawn*, *suspended*, or *superseded* by other documents, and therefore will not be
implemented by UNIC. See the [Unicode Technical Reports](http://www.unicode.org/reports/) page
for a complete list of these specifications and their current status.


### Naming Convention

The character properties defined in Unicode specifications follow a common naming convention.
Each character property and (non-numeric) property value has a name and an abbreviation.

The UNIC API for character properties is based on this convention and tries to stay as close as
possible to this naming schemes, making it easier to use the library when familiar with the
Unicode conventions.

NOTE: Since Rust does not support aliases for `enum` variants, only the *long names* are
supported in UNIC components. Property abbreviation names are provided in the documentation (to
help using them as variable-name, etc, if desired) and is also used in specific cases to prevent
namespace collision.

Example:

|                    | *Unicode Name*        | *UNIC Name*                                 |
|--------------------|-----------------------|---------------------------------------------|
| **Property**       | General_Category (gc) | `GeneralCategory`                           |
| **Property Value** | Uppercase_Letter (Lu) | `UppercaseLetter` / `is_uppercase_letter()` |
| **Property Value** | Cased_Letter (LC)     | `is_cased_letter()`                         |

In UNIC, the common way of accessing property values is using static function
`of()` on the property type (`enum`, `struct`). For example, *Some_Example*
property of a character will be available via `SomeExample::of(ch)`.

For property types with numeric values, the `number()` method will return the
numeric value. For example, `CanonicalCombiningClass::of(ch).number()` returns a
`u8` number.


### Unicode Character Database

The UCD [defines](http://www.unicode.org/reports/tr44/#Property_Index_Table)
various *character properties* for Unicode characters.

The following table shows their implementation status in UNIC.

| **Property Name** (abbr)        | **Property Type** | **UNIC Component**  | **UNIC Implementation**                             |
|---------------------------------|-------------------|---------------------|-----------------------------------------------------|
| **General**                     |                   |                     |                                                     |
| Age (age)                       | Catalog           | `unic-ucd-age`      | `enum Age { Assigned(UnicodeVersion), Unassigned }` |
| General_Category (gc)           | Enumeration       | `unic-ucd-category` | `enum GeneralCategory {...}`                        |
| ——                              |                   |                     |                                                     |
| **Bidirectional**               |                   |                     |                                                     |
| Bidi_Class                (bc)  | Enumeration       | `unic-ucd-bidi`     | `enum BidiClass {...}`                              |
| ——                              |                   |                     |                                                     |
| **Normalization**               |                   |                     |                                                     |
| Canonical_Combining_Class (ccc) | Enumeration       | `unic-ucd-normal`   | `struct CanonicalCombiningClass(u8)`                |
| Decomposition_Type (dt)         | Enumeration       | `unic-ucd-normal`   | `enum DecompositionType {...}`                      |


## Named Unicode Algorithms

Unicode [defines](http://www.unicode.org/versions/Unicode10.0.0/ch03.pdf) Named
Unicode Algorithms, that are specified in the Unicode Standard or in other
standards published by the Unicode Consortium.

The following table shows their implementation status in UNIC.

| **Name**                                       | **Reference** | **UNIC Component** |
|------------------------------------------------|---------------|--------------------|
| Canonical Ordering                             | Section 3.11  | `unic-ucd-normal`  |
| Canonical Composition                          | Section 3.11  | `unic-ucd-normal`  |
| Normalization                                  | Section 3.11  | `unic-ucd-normal`  |
| Hangul Syllable Composition                    | Section 3.12  | Not Public         |
| Hangul Syllable Decomposition                  | Section 3.12  | Not Public         |
| Hangul Syllable Name Generation                | Section 3.12  | Not Implemented    |
| Default Case Conversion                        | Section 3.13  | Not Implemented    |
| Default Case Detection                         | Section 3.13  | Not Implemented    |
| Default Caseless Matching                      | Section 3.13  | Not Implemented    |
| Bidirectional Algorithm (UBA)                  | UAX \#9       | `unic-bidi`        |
| Line Breaking Algorithm                        | UAX \#14      | Not Implemented    |
| Character Segmentation                         | UAX \#29      | Not Implemented    |
| Word Segmentation                              | UAX \#29      | Not Implemented    |
| Sentence Segmentation                          | UAX \#29      | Not Implemented    |
| Hangul Syllable Boundary Determination         | UAX \#29      | Not Implemented    |
| Standard Compression Scheme for Unicode (SCSU) | UTS \#6       | Not Implemented    |
| Unicode Collation Algorithm (UCA)              | UTS \#10      | Not Implemented    |
