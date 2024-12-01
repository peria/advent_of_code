#!/usr/bin/env python3

cubes = list(open("input/17.dat").read().splitlines())
#cubes = list(open("input/17s.dat").read().splitlines())
DIFFS = tuple((dx, dy, dz, dw)
              for dw in range(-1, 2)
              for dz in range(-1, 2)
              for dy in range(-1, 2)
              for dx in range(-1, 2))

actives = set()
neighbors = dict()
for y, cs in enumerate(cubes):
    for x, c in enumerate(cs):
        if c != '#':
            continue
        actives.add((x, y, 0, 0))
        for dx, dy, dz, dw in DIFFS:
            pos = (x + dx, y + dy, dz, dw)
            neighbors[pos] = neighbors.get(pos, 0) + 1


def do_activate(pos):
    return pos not in actives and neighbors[pos] == 3


def do_deactivate(pos):
    return pos in actives and (neighbors[pos] - 1) not in (2, 3)
    

for _ in range(6):
    activated = set(filter(do_activate, neighbors.keys()))
    deactivated = set(filter(do_deactivate, neighbors.keys()))

    for x, y, z, w in activated:
        for dx, dy, dz, dw in DIFFS:
            p = (x + dx, y + dy, z + dz, w + dw)
            neighbors[p] = neighbors.get(p, 0) + 1
    for x, y, z, w in deactivated:
        for dx, dy, dz, dw in DIFFS:
            p = (x + dx, y + dy, z + dz, w + dw)
            neighbors[p] = neighbors[p] - 1
    actives = (actives | activated) - deactivated
    neighbors = {k:v for k,v in neighbors.items() if v > 0}


print(len(actives))

