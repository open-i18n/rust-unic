#!/usr/bin/env python

# Copyright 2011-2013 The Rust Project Developers.
# Copyright 2015 The Servo Project Developers.
# Copyright 2017 The UNIC Project Developers.
#
# See the COPYRIGHT file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
# or http://opensource.org/licenses/MIT>, at your option. This file may not be
# copied, modified, or distributed except according to those terms.


import os
import sys

sys.path.append(os.path.join(os.path.dirname(__file__), "pylib"))
import common

from common import UCD_DATA_DIR, UCD_TEST_DATA_DIR


URL_PREFIX = 'http://www.unicode.org/Public/UCD/latest/ucd/'

DATA_FILES = [
    "DerivedAge.txt",
    "DerivedNormalizationProps.txt",
    "ReadMe.txt",
    "UnicodeData.txt",
]

TEST_DATA_FILES = [
    "BidiCharacterTest.txt",
    "BidiTest.txt",
    "NormalizationTest.txt",
]


def fetch_data(name):
    common.fetch(URL_PREFIX + name,
                 os.path.join(UCD_DATA_DIR, os.path.basename(name)))


def fetch_test_data(name):
    common.fetch(URL_PREFIX + name,
                 os.path.join(UCD_TEST_DATA_DIR, os.path.basename(name)))


if __name__ == "__main__":
    common.cleanup_data_dir(UCD_DATA_DIR)
    common.cleanup_data_dir(UCD_TEST_DATA_DIR)

    for name in DATA_FILES:
        fetch_data(name)
    for name in TEST_DATA_FILES:
        fetch_test_data(name)
