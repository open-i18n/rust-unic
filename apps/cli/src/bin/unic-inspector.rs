// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clap;

#[macro_use]
extern crate prettytable;

use clap::Arg;
use prettytable::format::TableFormat;
use prettytable::Table;

use unic::char::property::EnumeratedCharProperty;
use unic::ucd::{GeneralCategory, Name};

use std::time;
use std::thread;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};


fn main() {
    let app = app_from_crate!()
        .about(concat!(
            env!("CARGO_PKG_DESCRIPTION"),
            "\n\n",
            "Inspect characters and their properties",
        ))
        .arg(
            Arg::with_name("STRINGS")
                .help("Input strings (expected valid Unicode)")
                .required(false)
                .multiple(true),
        );
    let matches = app.get_matches();

    // == Read input ==
    let mut input: String = matches
        .values_of("STRINGS")
        .unwrap_or_default()
        .collect::<Vec<&str>>()
        .join(" ");

    if input.len() == 0 {
        let done = Arc::new(Mutex::new(false));
        let done_clone = done.clone();

        let handler = thread::spawn(move ||{
            let mut input = String::new();
            let _ = io::stdin().read_to_string(&mut input);

            *done_clone.lock().unwrap() = true;

            input
        });
        
        thread::sleep(time::Duration::from_millis(300));
        
        if *done.lock().unwrap() {
            input = handler.join().unwrap();
        }
    }

    // == Write output ==
    let mut table = Table::new();
    let mut table_format = TableFormat::new();
    table_format.padding(1, 1);
    table_format.column_separator('|');
    table.set_format(table_format);

    /* TODO: Enable with option
    table.set_titles(row![
        cu -> " Char ",
        cu -> " Code ",
        cu -> " Name ",
        cu -> " General_Category "
    ]);
    */

    input.chars().for_each(|chr| {
        let name = Name::of(chr)
            .map(|n| n.to_string())
            .unwrap_or_else(|| "<none>".to_owned());

        table.add_row(row![
            cb  -> &format!("{}", chr),
            rFr -> &format!("U+{:04X}", chr as u32),
            l   -> &name,
            l   -> &GeneralCategory::of(chr).long_name()
        ]);
    });

    table.printstd();
}
