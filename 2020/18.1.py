#!/usr/bin/env python3

import re

problems = open("input/18.dat").read().splitlines()
# problems = open("input/18s.dat").read().splitlines()


def solve(problem):
    m = re.match('(.*)\(([^()]+)\)(.*)', problem)
    if m:
        return solve(m[1] + str(solve(m[2])) + m[3])

    val = 0
    op = '+'
    for t in problem.split(' '):
        if t in ('+', '*'):
            op = t
        else:
            if op == '+':
                val += int(t)
            else:
                val *= int(t)
    return val
        

s = sum(solve(problem) for problem in problems)
print(s)
