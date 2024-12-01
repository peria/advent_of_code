#!/usr/bin/env python3

import re

problems = open("input/18.dat").read().splitlines()
#problems = open("input/18s.dat").read().splitlines()


def solve(problem):
    m = re.match('(.*)\(([^()]+)\)(.*)', problem)
    if m:
        return solve(m[1] + str(solve(m[2])) + m[3])

    tokens = [t if t in ('+', '*') else int(t) for t in problem.split(' ')]
    stack = []
    for t in tokens:
        if isinstance(t, int) and len(stack) > 0 and stack[-1] == '+':
            stack.pop()
            stack.append(stack.pop() + t)
        else:
            stack.append(t)

    tokens = stack
    stack = []
    for t in tokens:
        if isinstance(t, int):
            if len(stack) > 0:
                stack.append(stack.pop() * t)
            else:
                stack.append(t)
    assert(len(stack) == 1)
    return stack[0]
        

s = sum(solve(problem) for problem in problems)
print(s)
