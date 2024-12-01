#!/usr/bin/env python3

passports = ' '.join(open("input/04.txt").read().splitlines()).split('  ')
required_keys = set(['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']) # 'cid'

num_valid = 0
for passport in passports:
    kvs = passport.split(' ')
    keys = set([kv.split(':')[0] for kv in kvs])
    if len(required_keys - keys) == 0:
        num_valid += 1
print(num_valid)

