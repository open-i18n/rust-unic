// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code)]

//! # UNIC: Unicode and Internationalization Crates for Rust
//!
//! The `unic` super-crate (this) is a collection of all UNIC components, providing
//! an easy way of access to all functionalities, when all or many are needed,
//! instead of importing components one-by-one, and ensuring all components
//! imported are compatible in algorithms and consistent data-wise.
//!
//! ## Components
//!
//! -   [`ucd`](/unic-ucd): Unicode Character Database.
//!
//! -   [`bidi`](/unic-bidi): Unicode Bidirectional Algorithm (USA\#9).
//!
//! -   [`normal`](/unic-normal): Unicode Normalization Forms (USA\#15).
//!
//! -   [`idna`](/unic-idna): Unicode IDNA Compatibility Processing (UTS\#46).

pub extern crate unic_bidi as bidi;
pub extern crate unic_idna as idna;
pub extern crate unic_normal as normal;
pub extern crate unic_ucd as ucd;
