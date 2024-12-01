#!/usr/bin/env python3

sockets = sorted(list(map(int, open("input/10.dat").read().splitlines())))

sockets.append(sockets[-1] + 3)

out = 0
jolts = [0, 0, 0, 0]
for i in sockets:
    d = i - out
    jolts[d] += 1
    out = i

print(jolts[1] * jolts[3])

