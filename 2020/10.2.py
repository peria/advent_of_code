#!/usr/bin/env python3

sockets = sorted(list(map(int, open("input/10.dat").read().splitlines())))

sockets = [0] + sockets + [sockets[-1] + 3]
n = len(sockets)

_dp = [0] * n
_dp[0] = 1
def dp(i):
    if _dp[i] > 0:
        return _dp[i]
    for ii in range(1, 4):
        if i - ii >= 0 and sockets[i - ii] + 3 >= sockets[i]:
            _dp[i] += dp(i - ii)
    return _dp[i]

print(dp(n - 1))
