#!/usr/bin/env python3

numbers = list(map(int, open("input/15.dat").readline().split(',')))

memory = {n: i for i, n in enumerate(numbers)}

spoken = numbers[-1]
age = 0
for t in range(len(numbers), 30000000):
    if spoken not in memory:
        memory[spoken] = t
    spoken = age
    if age in memory:
        age = t - memory[age]
    else:
        age = 0
    memory[spoken] = t

print(spoken)
