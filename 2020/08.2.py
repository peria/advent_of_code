#!/usr/bin/env python3

code = list(map(lambda x: [x[0], int(x[1])],
                map(lambda x : x.split(' '),
                    open("input/08.dat").read().splitlines())))

for i in range(len(code)):
    if code[i][0] == 'acc':
        continue
    code[i][0] = 'nop' if code[i][0] == 'jmp' else 'jmp'

    acc = 0
    pc = 0
    last_jmp = -1
    visited = set()
    success = False
    while True:
        if pc > len(code) or pc < 0 or pc in visited:
            break
        if pc == len(code) and last_jmp == 1:
            success = True
            break

        op, imm = code[pc]
        visited.add(pc)
        if op == 'acc':
            acc += imm
        last_jmp = imm if op == 'jmp' else 1
        pc += last_jmp

    if success:
        print(acc)
        break

    code[i][0] = 'nop' if code[i][0] == 'jmp' else 'jmp'
