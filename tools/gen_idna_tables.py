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


import fileinput
import re
import os
import sys

from os.path import join
from collections import OrderedDict

sys.path.append(join(os.path.dirname(__file__), "pylib"))

import common
import rustout
import unicode_utils

from rustout import char_escape
from common import path, memoize
from unicode_utils import is_surrogate, cp_to_char


OUTPUT_DIRS = {
    'IDNA_MAPPING': path("unic/idna/mapping/src/tables"),
}


def data_path(name): return join(common.IDNA_DATA_DIR, name)


def test_data_path(name): return join(common.IDNA_TEST_DATA_DIR, name)


# == Version ==

@memoize
def get_unicode_version():
    with open(data_path("ReadMe.txt")) as readme_file:
        pattern = "for Version (\d+)\.(\d+)\.(\d+) of"
        return re.search(pattern, readme_file.read()).groups()


def emit_unicode_version(dir):
    with open(join(dir, 'unicode_version.rsv'), "w") as version_file:
        rustout.emit_value(
            __file__,
            version_file, get_unicode_version(),
            print_fun=lambda x: "UnicodeVersion { major: %s, minor: %s, micro: %s }" % x)


# == Map ==

@memoize
def get_idna_mapping_table():
    map = []
    map_string = OrderedDict({'': (0, 0)})
    map_string.offset = 0

    def map_string_slice(st):
        if st not in map_string:
            bytes = len(st.encode('utf8'))
            map_string[st] = (map_string.offset, bytes)
            map_string.offset += bytes
        return map_string[st]

    def print_slice(s):
        return "(StringTableSlice { byte_start: %d, byte_len: %d })" % map_string_slice(s)

    for line in fileinput.input(data_path("IdnaMappingTable.txt")):
        # remove comments
        line, _, _ = line.partition('#')

        # skip empty lines
        if len(line.strip()) == 0:
            continue

        fields = line.split(';')

        # skip surrogate codepoints; they don't occur in Rust strings
        if fields[0].strip() == 'D800..DFFF':
            continue

        first, _, last = fields[0].strip().partition('..')
        if not last:
            last = first
        first, last = int(first, 16), int(last, 16)

        # calc mapping
        mapping = rustout.title_case(fields[1])
        if len(fields) > 2:
            map_chars = fields[2].strip()
            if map_chars:
                unistr = u''.join(cp_to_char(int(hex, 16))
                                  for hex in map_chars.split(' '))
                mapping += print_slice(unistr)
            elif mapping == "Deviation":
                mapping += print_slice('')

        map.append((first, last, mapping))

    return (map, map_string)


def emit_idna_map_tables(dir):
    (map, map_string) = get_idna_mapping_table()

    with open(join(dir, 'idna_map.rsv'), "w") as map_file:
        rustout.emit_table(
            __file__,
            map_file,
            map,
            print_fun=lambda x: (
                "Range { from: '%s', to: '%s', mapping: %s }" % (
                    char_escape(x[0]),
                    char_escape(x[1]),
                    x[2],
                )
            ),
        )

    with open(join(dir, 'idna_map_string.rsv'), "w") as map_string_file:
        rustout.emit_strings(
            __file__,
            map_string_file,
            map_string.iterkeys(),
        )


# == Main ==

if __name__ == "__main__":
    common.cleanup_output_dirs(OUTPUT_DIRS.values())

    # IDNA
    emit_unicode_version(OUTPUT_DIRS['IDNA_MAPPING'])
    emit_idna_map_tables(OUTPUT_DIRS['IDNA_MAPPING'])
