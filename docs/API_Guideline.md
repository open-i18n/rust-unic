# UNIC API Guideline

UNIC tries to follow [Rust API guidelines](https://github.com/brson/rust-api-guidelines/). The
following checklist allows tracking the current status.

## Rust API conformance checklist

If the item is not applicable to the current code, it's left without a checkbox, to be
reviewed later if and when new applicable code is added.

Also, a checkbox is marked iff it applies to code in the project and is checked to be
satisfying.

- **Organization** *(crate is structured in an intelligible way)*
  - [X] Crate root re-exports common functionality ([C-REEXPORT])
  - [X] Modules provide a sensible API hierarchy ([C-HIERARCHY])

- **Naming** *(crate aligns with Rust naming conventions)*
  - [X] Casing conforms to RFC 430 ([C-CASE])
  - [X] Ad-hoc conversions follow `as_`, `to_`, `into_` conventions ([C-CONV])
  - Methods on collections that produce iterators follow `iter`, `iter_mut`, `into_iter` ([C-ITER])
  - Iterator type names match the methods that produce them ([C-ITER-TY])
  - Ownership suffixes use `_mut` and `_ref` ([C-OWN-SUFFIX])
  - Single-element containers implement appropriate getters ([C-GETTERS])

- **Interoperability** *(crate interacts nicely with other library functionality)*
  - [ ] Types eagerly implement common traits ([C-COMMON-TRAITS])
    - `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`,
        `Display`, `Default`
  - [ ] Conversions use the standard traits `From`, `AsRef`, `AsMut` ([C-CONV-TRAITS])
  - [ ] Collections implement `FromIterator` and `Extend` ([C-COLLECT])
  - [ ] Data structures implement Serde's `Serialize`, `Deserialize` ([C-SERDE])
  - [X] Crate has a `"serde"` cfg option that enables Serde ([C-SERDE-CFG])
  - Types are `Send` and `Sync` where possible ([C-SEND-SYNC])
  - Error types are `Send` and `Sync` ([C-SEND-SYNC-ERR])
  - Error types are meaningful, not `()` ([C-MEANINGFUL-ERR])
  - Binary number types provide `Hex`, `Octal`, `Binary` formatting ([C-NUM-FMT])

- **Macros** *(crate presents well-behaved macros)*
  - Input syntax is evocative of the output ([C-EVOCATIVE])
  - Macros compose well with attributes ([C-MACRO-ATTR])
  - Item macros work anywhere that items are allowed ([C-ANYWHERE])
  - Item macros support visibility specifiers ([C-MACRO-VIS])
  - Type fragments are flexible ([C-MACRO-TY])

- **Documentation** *(crate is abundantly documented)*
  - [ ] Crate level docs are thorough and include examples ([C-CRATE-DOC])
  - [ ] All items have a rustdoc example ([C-EXAMPLE])
  - [X] Examples use `?`, not `try!`, not `unwrap` ([C-QUESTION-MARK])
  - [ ] Function docs include error conditions in "Errors" section ([C-ERROR-DOC])
  - Function docs include panic conditions in "Panics" section ([C-PANIC-DOC])
  - [ ] Prose contains hyperlinks to relevant things ([C-LINK])
  - [X] Cargo.toml publishes CI badges for tier 1 platforms ([C-CI])
  - [X] Cargo.toml includes all common metadata ([C-METADATA])
    - authors, description, license, homepage, documentation, repository,
      readme, keywords, categories
  - Crate sets html_root_url attribute "https://docs.rs/$crate/$version" ([C-HTML-ROOT])
  - Cargo.toml documentation key points to "https://docs.rs/$crate" ([C-DOCS-RS])
  - [ ] Release notes document all significant changes ([C-RELNOTES])

- **Predictability** *(crate enables legible code that acts how it looks)*
  - Smart pointers do not add inherent methods ([C-SMART-PTR])
  - [X] Conversions live on the most specific type involved ([C-CONV-SPECIFIC])
  - [X] Functions with a clear receiver are methods ([C-METHOD])
  - [X] Functions do not take out-parameters ([C-NO-OUT])
  - [X] Operator overloads are unsurprising ([C-OVERLOAD])
  - Only smart pointers implement `Deref` and `DerefMut` ([C-DEREF])
  - `Deref` and `DerefMut` never fail ([C-DEREF-FAIL])
  - [ ] Constructors are static, inherent methods ([C-CTOR])

- **Flexibility** *(crate supports diverse real-world use cases)*
  - [X] Functions expose intermediate results to avoid duplicate work ([C-INTERMEDIATE])
  - [ ] Caller decides where to copy and place data ([C-CALLER-CONTROL])
  - [ ] Functions minimize assumptions about parameters by using generics ([C-GENERIC])
  - Traits are object-safe if they may be useful as a trait object ([C-OBJECT])

- **Type safety** *(crate leverages the type system effectively)*
  - [X] Newtypes provide static distinctions ([C-NEWTYPE])
  - [ ] Arguments convey meaning through types, not `bool` or `Option` ([C-CUSTOM-TYPE])
  - [ ] Types for a set of flags are `bitflags`, not enums ([C-BITFLAG])
  - [ ] Builders enable construction of complex values ([C-BUILDER])

- **Dependability** *(crate is unlikely to do the wrong thing)*
  - [X] Functions validate their arguments ([C-VALIDATE])
  - [X] Destructors never fail ([C-DTOR-FAIL])
  - [X] Destructors that may block have alternatives ([C-DTOR-BLOCK])

- **Debuggability** *(crate is conducive to easy debugging)*
  - [ ] All public types implement `Debug` ([C-DEBUG])
  - [ ] `Debug` representation is never empty ([C-DEBUG-NONEMPTY])

- **Future proofing** *(crate is free to improve without breaking users' code)*
  - [X] Structs have private fields ([C-STRUCT-PRIVATE])
  - [X] Newtypes encapsulate implementation details ([C-NEWTYPE-HIDE])

- **Necessities** *(to whom they matter, they really matter)*
  - [X] Public dependencies of a stable crate are stable ([C-STABLE])
  - [X] Crate and its dependencies have a permissive license ([C-PERMISSIVE])
