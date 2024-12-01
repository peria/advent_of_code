#!/usr/bin/env python3

answers = ' '.join(open("input/06.dat").read().splitlines()).split('  ')

def count(answer):
    ans = set(list(''.join(answer.split(' '))))
    return len(ans)

print(sum(map(count, answers)))
