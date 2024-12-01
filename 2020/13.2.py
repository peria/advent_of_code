#!/usr/bin/env python3

_, services = open("input/13.dat").read().splitlines()
departs = services.split(',')

diffs = []
for i, p in enumerate(departs):
    if p == 'x':
        continue
    p = int(p)
    diffs.append([p, (p - i % p) % p])

prod = 1
for p, _ in diffs:
    prod *= p

n = sum([r * (prod // p) * pow(prod // p, p - 2, p) for p, r in diffs]) % prod
print(n)
