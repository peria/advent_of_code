#!/usr/bin/env python3

import itertools
from functools import reduce

program = list(map(lambda x: x.split(' = '), open("input/14.dat").read().splitlines()))

memory = {}
mask = None
for key, val in program:
    if key == 'mask':
        on, x, bits = (0, 0, [])
        smask = val
        bit = 1
        for v in reversed(val):
            if v == '1':
                on |= bit
            elif v == 'X':
                x |= bit
                bits.append(bit)
            bit *= 2
        masks = [0]
        for i in range(1, len(bits) + 1):
            for bs in itertools.combinations(bits, i):
                mask = reduce(lambda x, y: x | y, bs)
                masks.append(mask)
    else:
        a = (int(key[4:-1]) | on | x) ^ x
        for mask in masks:
            memory[a | mask] = int(val)

print(sum(v for v in memory.values()))
    
