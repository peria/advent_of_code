#!/usr/bin/env python3

numbers = list(map(int, open("input/15.dat").readline().split(',')))

memory = {n: [i] for i, n in enumerate(numbers)}

spoken = numbers[-1]
age = 0
for t in range(len(numbers), 2020):
    if spoken not in memory:
        memory[spoken] = [t]
    spoken = age
    if age in memory:
        age = t - memory[age][-1]
    else:
        memory[age] = []
        age = 0
    memory[spoken].append(t)

print(spoken)
