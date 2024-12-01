#!/usr/bin/env python3

num_valid = 0
while True:
    try:
        line = input()
    except EOFError:
        break

    lims, c, password = line.split(' ')
    lim_min, lim_max = map(int, lims.split('-'))
    num = password.count(c[0])
    if lim_min <= num and num <= lim_max:
        num_valid += 1
print(num_valid)

