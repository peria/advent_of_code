#!/usr/bin/env python3

num_valid = 0
while True:
    try:
        line = input()
    except EOFError:
        break

    indexes, c, password = line.split(' ')
    indexes = list(map(int, indexes.split('-')))
    to_check = [password[i - 1] for i in indexes]
    if to_check.count(c[0]) == 1:
        num_valid += 1
print(num_valid)

