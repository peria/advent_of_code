#!/usr/bin/env python3

import re

rules = {}
for k, v in map(lambda x : x.split(' bags contain '), open("input/07.dat").read().splitlines()):
    inners = {}
    for inner in v.split(', '):
        m = re.match('(\d)+ (.+) bag', inner)
        if m:
            n, c = int(m[1]), m[2]
            inners[c] = n
    rules[k] = inners

bugs = {}
unpacking_bugs = {'shiny gold': 1}
while len(unpacking_bugs) > 0:
    new_bugs = {}
    for bug, k in unpacking_bugs.items():
        for color, n in rules[bug].items():
            new_bugs[color] = new_bugs.get(color, 0) + n * k
    for c, n in new_bugs.items():
        bugs[c] = bugs.get(c, 0) + n
    unpacking_bugs = new_bugs

print(sum(bugs.values()))
