#!/usr/bin/env python3

cubes = list(open("input/17.dat").read().splitlines())
#cubes = list(open("input/17s.dat").read().splitlines())
DIFFS = tuple((dx, dy, dz) for dz in range(-1, 2) for dy in range(-1, 2) for dx in range(-1, 2))

actives = set()
neighbors = dict()
for y, cs in enumerate(cubes):
    for x, c in enumerate(cs):
        if c != '#':
            continue
        actives.add((x, y, 0))
        for dx, dy, dz in DIFFS:
            pos = (x + dx, y + dy, dz)
            neighbors[pos] = neighbors.get(pos, 0) + 1


def do_activate(pos):
    return pos not in actives and neighbors[pos] == 3


def do_deactivate(pos):
    return pos in actives and (neighbors[pos] - 1) not in (2, 3)


def display(ps):
    xs, ys, zs = list(map(list, zip(*ps)))
    for y in range(min(ys), max(ys) + 1):
        l = ''
        for z in range(min(zs), max(zs) + 1):
            for x in range(min(xs), max(xs) + 1):
                l += '#' if (x, y, z) in ps else '.'
            l += '  '
        print(l)
    

for _ in range(6):
    activated = set(filter(do_activate, neighbors.keys()))
    deactivated = set(filter(do_deactivate, neighbors.keys()))

    # display(actives)
    # print(actives)
    for x, y, z in activated:
        for dx, dy, dz in DIFFS:
            p = (x + dx, y + dy, z + dz)
            neighbors[p] = neighbors.get(p, 0) + 1
    for x, y, z in deactivated:
        for dx, dy, dz in DIFFS:
            p = (x + dx, y + dy, z + dz)
            neighbors[p] = neighbors[p] - 1
    actives = (actives | activated) - deactivated
    neighbors = {k:v for k,v in neighbors.items() if v > 0}


print(len(actives))

