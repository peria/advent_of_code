#!/usr/bin/env python3

now, services = open("input/13.dat").read().splitlines()
now = int(now)
services = list(map(int, filter(lambda x: x != 'x', services.split(','))))

bus = min(([i, i - now % i] for i in services), key=lambda x: x[1])
print(bus[0] * bus[1])
