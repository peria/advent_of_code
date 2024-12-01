#!/usr/bin/env python3

import re

patterns, messages = open("input/19.dat").read().split('\n\n')
patterns, messages = open("input/19s2.dat").read().split('\n\n')

def parse_pattern(x):
    t, p = x.split(': ')
    if '"' in p:
        return t, p[1]
    if t == '8':
        p = '42 | 42 8'
    elif t == '11':
        p = '42 31 | 42 11 31'
    p = list(map(lambda x: x.split(' '), list(p.split(' | ')) if '|' in p else [p]))
    return t, p

patterns = dict(map(parse_pattern, patterns.split('\n')))
messages = list(messages.strip().split('\n'))

def do_match(msg, i, label):
    pattern = patterns[label]

    if i >= len(msg):
        return False, -1

    if isinstance(pattern, str):
        return msg[i] == pattern, i + 1

    for ptn in pattern:
        succeed = True
        j = i
        for t in ptn:
            r, j = do_match(msg, j, t)
            if not r:
                succeed = False
                break
        if succeed:
            return True, j
    return False, -1


def is_valid(msg):
    r, i = do_match(msg, 0, '0')
    return r and i == len(msg)


for msg in messages:
    if is_valid(msg):
        print(msg)

print(sum([is_valid(msg) for msg in messages]))
