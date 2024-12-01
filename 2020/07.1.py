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

inner_rules = {k: set() for k in rules.keys()}
for k, cs in rules.items():
    for c in cs.keys():
        inner_rules[c].add(k)

colors = set()
q = set(['shiny gold'])
while len(q) > 0:
    q2 = set()
    for c in q:
        for out in inner_rules[c]:
            if out in colors:
                continue
            colors.add(out)
            q2.add(out)
    q = q2

print(len(colors))
