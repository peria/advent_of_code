#!/usr/bin/env python3

import re

actions = list(map(lambda x: (x[0], int(x[1:])), open("input/12.dat").read().splitlines()))

DIFF = ((1, 0), (0, 1), (-1, 0), (0, -1))

position = [0, 0]  # x, y
direction = 0  # 0: East, 1: North, 2: West, 3: South
for action, value in actions:
    if action == 'N':
        position[1] += value
    elif action == 'S':
        position[1] -= value
    elif action == 'E':
        position[0] += value
    elif action == 'W':
        position[0] -= value
    elif action == 'L':
        direction = (direction + value // 90) % 4
    elif action == 'R':
        direction = (direction + 360 - value // 90) % 4
    else:
        diff = DIFF[direction]
        position[0] += diff[0] * value
        position[1] += diff[1] * value
print(position)
print(abs(position[0]) + abs(position[1]))

