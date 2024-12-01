#!/usr/bin/env python3

import itertools

numbers = list(map(int, open("input/09.dat").read().splitlines()))

for i in range(25, len(numbers)):
    number = numbers[i]
    preambles = numbers[i - 25: i]
    
    def is_valid(x, ys):
        return any(y + z == x for y, z in itertools.combinations(ys, 2))

    if not is_valid(number, preambles):
        print(number)

