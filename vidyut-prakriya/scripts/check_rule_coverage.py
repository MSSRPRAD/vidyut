#!/usr/bin/env python3
from pathlib import Path
import re
import glob

RULE_OK = "✅"
RULE_UNTESTED = "⚠️ "
RULE_MISSING = "❌"

def print_legend():
    print("===== Legend ======")
    print(f"{RULE_OK}\t\tSutra is tested.")
    print(f"{RULE_UNTESTED}\t\tSutra is implemented but untested.")
    print(f"{RULE_MISSING}\t\tSutra is missing.")
    print()
    print("These statuses are heuristics. Verify them by checking the underlying code.")
    print("===================")

base = Path(__file__).parent.parent
src = base / "src"
tests = base / "tests"

all_rules = []
with open(base / "data/sutrapatha.tsv") as f:
    for line in f:
        code, text = line.split('\t')
        all_rules.append(code)


implemented_rules = set()
for path in glob.glob("**/*.rs", root_dir=src, recursive=True):
    with open(src / path) as f:
        for line in f:
            if m := re.search(r'"(\d+\.\d+\.\d+)', line):
                implemented_rules.add(m.group(1))

tested_rules = set()
for path in glob.glob("**/*.rs", root_dir=tests, recursive=True):
    with open(tests / path) as f:
        for line in f:
            if m := re.search(r"(\d+_\d+_\d+)", line):
                tested_rules.add(m.group(1).replace('_', '.'))

print_legend()
num_ok = 0
num_untested = 0
num_missing = 0
for rule in all_rules:
    status = None
    if rule in tested_rules:
        status = RULE_OK
        num_ok += 1
    elif rule in implemented_rules:
        status = RULE_UNTESTED
        num_untested += 1
    else:
        status = RULE_MISSING
        num_missing += 1
    print(f"{status}\t\t{rule}")
print_legend()
print(f"Num tested   : {num_ok}")
print(f"Num untested : {num_untested}")
print(f"Num missing  : {num_missing}")