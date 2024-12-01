#!/usr/bin/env python3

rules, yours, nearbys = open("input/16.dat").read().split('\n\n')

def parse_rules(rule):
    title, rules = rule.split(': ')
    rules = list(map(lambda x: list(map(int, x.split('-'))), rules.split(' or ')))
    
    return title, rules


rules = dict(map(parse_rules, rules.splitlines()))
yours = list(map(int, yours.splitlines()[-1].split(',')))
nearbys = map(lambda x: list(map(int, x.split(','))), nearbys.splitlines()[1:])


def is_valid(n, rule):
    assert isinstance(rule, list)
    assert isinstance(rule[0], list)
    return any(r[0] <= n and n <= r[1] for r in rule)


def is_valid_ticket(ticket):
    return all(any(is_valid(n, rule) for rule in rules.values()) for n in ticket)

tickets = list(filter(is_valid_ticket, nearbys))

titles = []
for i in range(len(rules)):
    candidates = set()
    for t, rs in rules.items():
        if all(is_valid(ticket[i], rs) for ticket in tickets):
            candidates.add(t)
    titles.append(candidates)

updated = True
while updated:
    updated = False
    for i, candidates in enumerate(titles):
        if isinstance(candidates, str):
            continue
        if len(candidates) == 1:
            title = list(candidates)[0]
            titles[i] = title
            for j in range(len(titles)):
                if isinstance(titles[j], set) and title in titles[j]:
                    titles[j].remove(title)
                    updated = True

prod = 1
for i, title in enumerate(titles):
    if not title.startswith('departure'):
        continue
    prod *= yours[i]
print(prod)

