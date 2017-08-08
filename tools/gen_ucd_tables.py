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

sys.path.append(join(os.path.dirname(__file__), "pylib"))

import common
import rustout
import unicode_utils

from common import path, memoize
from unicode_utils import is_surrogate


OUTPUT_DIRS = {
    'UCD_NORMAL': path("unic/ucd/normal/src/tables"),
}


def data_path(name): return join(common.UCD_DATA_DIR, name)


def test_data_path(name): return join(common.UCD_TEST_DATA_DIR, name)


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


# == Shared ==

@memoize
def get_unicode_data():
    unicode_data = {}
    range_start = None
    for line in fileinput.input(data_path("UnicodeData.txt")):
        data = line.split(";")
        if len(data) != 15:
            continue
        cp = int(data[0], 16)
        if is_surrogate(cp):
            continue
        if range_start != None:
            for i in xrange(range_start, cp):
                unicode_data[i] = data
            range_start = None
        if data[1].endswith(", First>"):
            range_start = cp
            continue
        unicode_data[cp] = data
    return unicode_data


# == Gen_Cat ==

# Mapping taken from Table 12 from:
# http://www.unicode.org/reports/tr44/#General_Category_Values
EXPANDED_CATEGORIES = {
    'Lu': ['LC', 'L'], 'Ll': ['LC', 'L'], 'Lt': ['LC', 'L'],
    'Lm': ['L'], 'Lo': ['L'],
    'Mn': ['M'], 'Mc': ['M'], 'Me': ['M'],
    'Nd': ['N'], 'Nl': ['N'], 'No': ['No'],
    'Pc': ['P'], 'Pd': ['P'], 'Ps': ['P'], 'Pe': ['P'],
    'Pi': ['P'], 'Pf': ['P'], 'Po': ['P'],
    'Sm': ['S'], 'Sc': ['S'], 'Sk': ['S'], 'So': ['S'],
    'Zs': ['Z'], 'Zl': ['Z'], 'Zp': ['Z'],
    'Cc': ['C'], 'Cf': ['C'], 'Cs': ['C'], 'Co': ['C'], 'Cn': ['C'],
}


# == Normal ==

@memoize
def get_normal_props():
    interestingprops = ["Full_Composition_Exclusion"]

    props = {}
    re1 = re.compile("^ *([0-9A-F]+) *; *(\w+)")
    re2 = re.compile("^ *([0-9A-F]+)\.\.([0-9A-F]+) *; *(\w+)")

    for line in fileinput.input(data_path("DerivedNormalizationProps.txt")):
        prop = None
        d_lo = 0
        d_hi = 0
        m = re1.match(line)
        if m:
            d_lo = m.group(1)
            d_hi = m.group(1)
            prop = m.group(2)
        else:
            m = re2.match(line)
            if m:
                d_lo = m.group(1)
                d_hi = m.group(2)
                prop = m.group(3)
            else:
                continue
        if interestingprops and prop not in interestingprops:
            continue
        d_lo = int(d_lo, 16)
        d_hi = int(d_hi, 16)
        if prop not in props:
            props[prop] = []
        props[prop].append((d_lo, d_hi))

    # optimize if possible
    for prop in props:
        props[prop] = ranges_from_codepoints(
            codepoints_from_ranges(props[prop]))

    return props


@memoize
def get_normal_form_info():
    unicode_data = get_unicode_data()

    canonical_combining_class = {}
    canonical_decomposition_mapping = {}
    compatibility_decomposition_mapping = {}
    compatibility_decomposition_type = {}
    general_category_mark = []

    for cp in unicode_data:
        [_, name, gen_cat, ccc, bidi_class, decomp, deci, digit, num, mirror,
         old, iso, upcase, lowcase, titlecase] = unicode_data[cp]

        # GC=Mark
        if 'M' in [gen_cat] + EXPANDED_CATEGORIES.get(gen_cat, []):
            general_category_mark.append(cp)

        # record combining class, if any
        if ccc != "0":
            if ccc not in canonical_combining_class:
                canonical_combining_class[ccc] = []
            canonical_combining_class[ccc].append(cp)

        # store decomposition, if given
        if decomp != "":
            tokens = decomp.split()
            if tokens[0].startswith('<'):
                decomp_type = tokens[0][1:-1].title()
                if decomp_type not in compatibility_decomposition_type:
                    compatibility_decomposition_type[decomp_type] = []
                compatibility_decomposition_type[decomp_type].append(cp)
                compatibility_decomposition_mapping[cp] = [
                    int(x, 16)
                    for x in tokens[1:]
                ]
            else:
                # decomp_type = 'Canonical'
                canonical_decomposition_mapping[cp] = [
                    int(x, 16)
                    for x in tokens
                ]

    general_category_mark = ranges_from_codepoints(general_category_mark)
    canonical_combining_class = range_value_triplets_from_codepoints(
        canonical_combining_class)
    compatibility_decomposition_type = range_value_triplets_from_codepoints(
        compatibility_decomposition_type)

    return (
        general_category_mark,
        canonical_combining_class,
        canonical_decomposition_mapping,
        compatibility_decomposition_mapping,
        compatibility_decomposition_type,
    )


@memoize
def get_canonical_composition_mapping():
    (_, _, canonical_decomposition_mapping, _, _) = get_normal_form_info()

    mapping = {}
    composition_exclusions = get_normal_props()["Full_Composition_Exclusion"]
    for char in canonical_decomposition_mapping.keys():
        if True in map(lambda (start, end): start <= char <= end, composition_exclusions):
            continue
        decomp = canonical_decomposition_mapping[char]
        if len(decomp) == 2:
            if not mapping.has_key(decomp[0]):
                mapping[decomp[0]] = []
            mapping[decomp[0]].append((decomp[1], char))
    return mapping


def emit_normal_form_tables(dir):
    (
        general_category_mark,
        canonical_combining_class,
        canonical_decomposition_mapping,
        compatibility_decomposition_mapping,
        compatibility_decomposition_type,
    ) = get_normal_form_info()

    with open(join(dir, 'general_category_mark.rsv'), "w") as values_file:
        rustout.emit_table(
            __file__,
            values_file,
            general_category_mark,
            print_fun=lambda x: "(%s, %s)" % (
                rustout.char_literal(x[0]),
                rustout.char_literal(x[1]),
            ),
        )

    with open(join(dir, 'canonical_combining_class_values.rsv'), "w") as values_file:
        rustout.emit_table(
            __file__,
            values_file,
            canonical_combining_class,
            print_fun=lambda x: "(%s, %s, CanonicalCombiningClass(%s))" % (
                rustout.char_literal(x[0]),
                rustout.char_literal(x[1]),
                x[2],
            ),
        )

    with open(join(dir, 'compatibility_decomposition_type_values.rsv'), "w") as values_file:
        rustout.emit_table(
            __file__,
            values_file,
            compatibility_decomposition_type,
            print_fun=lambda x: "(%s, %s, %s)" % (
                rustout.char_literal(x[0]),
                rustout.char_literal(x[1]),
                x[2],
            ),
        )

    with open(join(dir, 'canonical_decomposition_mapping_lookup.rsv'), "w") as lookup_file, \
            open(join(dir, 'canonical_decomposition_mapping_values.rsv'), "w") as values_file:
        rustout.emit_lookup_tables(
            __file__,
            lookup_file,
            values_file,
            canonical_decomposition_mapping,
        )

    with open(join(dir, 'compatibility_decomposition_mapping_lookup.rsv'), "w") as lookup_file, \
            open(join(dir, 'compatibility_decomposition_mapping_values.rsv'), "w") as values_file:
        rustout.emit_lookup_tables(
            __file__,
            lookup_file,
            values_file,
            compatibility_decomposition_mapping,
        )

    with open(join(dir, 'canonical_composition_mapping_lookup.rsv'), "w") as lookup_file, \
            open(join(dir, 'canonical_composition_mapping_values.rsv'), "w") as values_file:
        canonical_composition_mapping = get_canonical_composition_mapping()
        rustout.emit_lookup_tables(
            __file__,
            lookup_file,
            values_file,
            canonical_composition_mapping,
            value_fun=lambda char: sorted(
                canonical_composition_mapping[char], lambda x, y: x[0] - y[0]
            ),
            value_print_fun=lambda pair: "(%s, %s)" % (
                rustout.char_literal(pair[0]),
                rustout.char_literal(pair[1]),
            ),
        )


# == Misc ==

def range_value_triplets_from_codepoints(groups):
    list = [
        (x, y, value)
        for value in groups
        for (x, y) in ranges_from_codepoints(groups[value])
    ]
    list.sort(key=lambda x: x[0])
    return list


def range_value_triplets_from_ranges(groups):
    list = [
        (x, y, value)
        for value in groups
        for (x, y) in groups[value]
    ]
    list.sort(key=lambda x: x[0])
    return list


def ranges_from_codepoints(codepoints):
    ranges = []
    codepoitns = sorted(set(codepoints))
    cur_start = codepoitns.pop(0)
    cur_end = cur_start
    for codepoint in codepoitns:
        assert codepoint > cur_end, \
            "cur_end: %s, codepoint: %s" % (hex(cur_end), hex(codepoint))
        if codepoint == cur_end + 1:
            cur_end = codepoint
        else:
            ranges.append((cur_start, cur_end))
            cur_start = cur_end = codepoint
    ranges.append((cur_start, cur_end))
    return ranges


def codepoints_from_ranges(ranges):
    return [cp for (start, end) in ranges for cp in xrange(start, end + 1)]


# == Main ==

if __name__ == "__main__":
    # Normal
    emit_unicode_version(OUTPUT_DIRS['UCD_NORMAL'])
    emit_normal_form_tables(OUTPUT_DIRS['UCD_NORMAL'])
