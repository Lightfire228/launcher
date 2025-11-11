#!/bin/env python

from subprocess import run
from shutil import which
from pathlib import Path

CARGO = which('cargo') or ''

bin = Path.home() / 'bin/fourth_bridge'

def main():
    _ = run([CARGO, 'build', '--release'])

    _ = bin.write_bytes(Path('./target/release/fourth_bridge').read_bytes())

    bin.chmod(0x744)




if __name__ == '__main__':
    main()
