#!/usr/bin/env python3

mp = []
while True:
    try:
        line = input()
    except EOFError:
        break
    mp.append(line)

h = len(mp)
w = len(mp[0])

result = 1
for dx, dy in [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]:
    count = 0
    x, y = 0, 0
    while y < h:
        count += 1 if mp[y][x] == '#' else 0
        x = (x + dx) % w
        y = y + dy
    print(count)
    result *= count
print(result)
