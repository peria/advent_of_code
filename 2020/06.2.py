#!/usr/bin/env python3

import functools

answers = ' '.join(open("input/06.dat").read().splitlines()).split('  ')

def count(answers):
    return len(functools.reduce(lambda a, b: set(list(a)) & set(list(b)), answers.split(' ')))

print(sum(map(count, answers)))
