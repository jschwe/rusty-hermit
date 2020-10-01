#!/usr/bin/env python3

import sys
from subprocess import call
from pathlib import Path
import os

location = Path(os.path.realpath(__file__)).parent.parent.parent
print("Located at: {}".format(location))
runner = ["python3", "{}/libhermit-rs/tests/hermit_test_runner.py".format(location)]
runner.extend(sys.argv[1:])
print("Args: {}".format(runner))
call( runner )