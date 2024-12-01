#!/usr/bin/env python3

seats = list(open("input/11.dat").read().splitlines())
h = len(seats)
w = len(seats[0])
DIJS = ((-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1))

def count_occupied(s, i, j):
    def can_see_occupied(s, r, c, dr, dc):
        r += dr
        c += dc
        while r >= 0 and c >= 0 and r < h and c < w:
            if s[r][c] == '.':
                r += dr
                c += dc
                continue
            return s[r][c] == '#'
        return False

    return sum(int(can_see_occupied(s, i, j, di, dj)) for di, dj in DIJS)


while True:
    seats2 = []
    updated = False
    for i in range(h):
        line = ''
        for j in range(w):
            seat = seats[i][j]
            seat2 = seat
            if seat == 'L' and count_occupied(seats, i, j) == 0:
                seat2 = '#'
            elif seats[i][j] == '#' and count_occupied(seats, i, j) >= 5:
                seat2 = 'L'
            if seat != seat2:
                updated = True
            line = line + seat2
        seats2.append(line)
    seats = seats2
    if not updated:
        break
    # print('\n'.join(seats + ['']))

print(sum(l.count('#') for l in seats))
