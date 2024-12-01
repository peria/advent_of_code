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

x = 0
count = 0
for y in range(h):
    count += 1 if mp[y][x] == '#' else 0
    x = (x + 3) % w
print(count)
