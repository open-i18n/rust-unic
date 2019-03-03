#!/usr/bin/env bash

# Copyright 2017 The UNIC Project Developers.
#
# See the COPYRIGHT file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -e


# List of components, in order of dependency
export COMPONENTS="
    unic/common

    unic/char/range

    unic/char/basics
    unic/char/property

    unic/char

    unic/ucd/version

    unic/ucd/category
    unic/ucd/hangul

    unic/ucd/age
    unic/ucd/bidi
    unic/ucd/block
    unic/ucd/case
    unic/ucd/common
    unic/ucd/ident
    unic/ucd/name
    unic/ucd/name_aliases
    unic/ucd/normal
    unic/ucd/segment

    unic/ucd

    unic/bidi

    unic/normal

    unic/segment

    unic/idna/mapping
    unic/idna/punycode
    unic/idna

    unic/emoji/char
    unic/emoji

    unic
"

# List of apps, in order of dependency
export APPS="
    apps/cli
"

-() {
    cmd="$@"
    echo
    echo "`tput bold; tput setaf 4`============`tput sgr0` "
    echo "`tput bold; tput setaf 4`   Executing`tput sgr0` $cmd"
    echo
    $cmd
    echo "`tput bold; tput setaf 4`   Succeeded`tput sgr0` $cmd"
    echo "`tput bold; tput setaf 4`------------`tput sgr0` "
    echo
}
