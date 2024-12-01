#!/usr/bin/env python3

program = list(map(lambda x: x.split(' = '), open("input/14.dat").read().splitlines()))

memory = {}
mask = None
for key, val in program:
    if key == 'mask':
        on, off = (0, 2**36-1)
        bit = 1
        for v in reversed(val):
            if v == '1':
                on |= bit
            elif v == '0':
                off ^= bit
            bit *= 2
    else:
        addr = int(key[4:-1])
        v = int(val)
        v = (v | on) & off
        memory[addr] = v

print(sum(v for v in memory.values()))
    
