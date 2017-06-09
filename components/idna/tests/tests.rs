// Copyright 2016 The rust-url developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_idna;
extern crate rustc_serialize;
extern crate test;

mod conformance_tests;

fn main() {
    let mut tests = Vec::new();
    {
        let mut add_test = |name, run| {
            tests.push(test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::DynTestName(name),
                    ignore: false,
                    should_panic: test::ShouldPanic::No,
                },
                testfn: run,
            })
        };
        conformance_tests::collect_tests(&mut add_test);
    }
    test::test_main(&std::env::args().collect::<Vec<_>>(), tests)
}
