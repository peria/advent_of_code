#!/usr/bin/env python3

code = list(map(lambda x: (x[0], int(x[1])),
                map(lambda x : x.split(' '),
                    open("input/08.dat").read().splitlines())))

acc = 0
pc = 0
visited = set()
while pc not in visited:
    op, imm = code[pc]
    visited.add(pc)
    if op == 'acc':
        acc += imm
    pc += imm if op == 'jmp' else 1
print(acc)
