#!/usr/bin/env python3

rules, yours, nearbys = open("input/16.dat").read().split('\n\n')

def parse_rules(rule):
    title, rules = rule.split(': ')
    rules = list(map(lambda x: list(map(int, x.split('-'))), rules.split(' or ')))
    
    return [title, rules]

rules = list(map(parse_rules, rules.splitlines()))
yours = list(map(int, yours.splitlines()[-1].split(',')))
nearbys = map(lambda x: list(map(int, x.split(','))), nearbys.splitlines()[1:])

def is_valid(n, rule):
    assert len(rule) == 2
    for r in rule[1]:
        if r[0] <= n and n <= r[1]:
            return True
    return False


def sum_invalid_nums(ticket):
    s = 0
    for n in ticket:
        if all(not is_valid(n, rule) for rule in rules):
            s += n
    return s

s = 0
for ticket in nearbys:
    s += sum_invalid_nums(ticket)
print(s)

