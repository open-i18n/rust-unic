# Copyright 2017 The UNIC Project Developers.
#
# See the COPYRIGHT file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
# or http://opensource.org/licenses/MIT>, at your option. This file may not be
# copied, modified, or distributed except according to those terms.


import os
import shutil


ROOT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))


def path(rel_path):
    return os.path.join(ROOT_DIR, *(rel_path.split("/")))


UCD_DATA_DIR = path("data/ucd")
UCD_TEST_DATA_DIR = path("data/ucd/test")

IDNA_DATA_DIR = path("data/idna")
IDNA_TEST_DATA_DIR = path("data/idna/test")


def cleanup_data_dir(dir):
    if os.path.exists(dir):
        shutil.rmtree(dir)
    os.makedirs(dir)


def cleanup_output_dirs(dirs):
    for dir in dirs:
        for filename in os.listdir(dir):
            if filename.endswith(".rsv"):
                os.remove(os.path.join(dir, filename))


def fetch(url, dst):
    os.system("curl -o '%s' '%s'" % (dst, url))
    if not os.path.exists(dst):
        sys.stderr.write("cannot fetch %s" % name)
        exit(1)


def memoize(function):
    store = {}

    def wrapper(*args):
        if args not in store:
            store[args] = function(*args)
        return store[args]
    return wrapper
