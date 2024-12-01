#!/usr/bin/env python3

import itertools

numbers = list(map(int, open("input/09.dat").read().splitlines()))
target = 104054607

i = 0
j = 1
s = numbers[0]
while i < len(numbers):
    if s == target:
        r = numbers[i: j]
        # print(r)
        print(min(r) + max(r))
        break
    elif s < target:
        s += numbers[j]
        j += 1
    else:
        s -= numbers[i]
        i += 1

