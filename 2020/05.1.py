#!/usr/bin/env python3

seats = open("input/05.txt").read().splitlines()

def seat_to_id(seat):
    ret = 0
    def c_to_num(c):
        if c == 'B' or c == 'R':
            return 1
        else:
            return 0

    for c in seat:
        ret = ret * 2 + c_to_num(c)
    return ret

print(max(map(seat_to_id, seats)))
