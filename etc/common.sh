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
    unic/char/property
    unic/char/range
    unic/char

    unic/ucd/version
    unic/ucd/age
    unic/ucd/bidi
    unic/ucd/case
    unic/ucd/category
    unic/ucd/common
    unic/ucd/ident
    unic/ucd/name
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
