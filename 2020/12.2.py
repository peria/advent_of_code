#!/usr/bin/env python3

import re

actions = list(map(lambda x: (x[0], int(x[1:])), open("input/12.dat").read().splitlines()))

ship = [0, 0]  # x, y
waypoint = [10, 1]
for action, value in actions:
    if action == 'N':
        waypoint[1] += value
    elif action == 'S':
        waypoint[1] -= value
    elif action == 'E':
        waypoint[0] += value
    elif action == 'W':
        waypoint[0] -= value
    elif action == 'L':
        for _ in range(value // 90):
            waypoint = [-waypoint[1], waypoint[0]]
    elif action == 'R':
        for _ in range(value // 90):
            waypoint = [waypoint[1], -waypoint[0]]
    else:
        ship[0] += waypoint[0] * value
        ship[1] += waypoint[1] * value
print(ship)
print(abs(ship[0]) + abs(ship[1]))

