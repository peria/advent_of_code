#!/usr/bin/env python3

import re

# four digits; at least 1920 and at most 2002.
def validateBirthYear(value):
    if not re.match('^\d{4}$', value):
        return False
    num = int(value)
    return 1920 <= num and num <= 2002

# four digits; at least 2010 and at most 2020.
def validateIssueYear(value):
    if not re.match('^\d{4}$', value):
        return False
    num = int(value)
    return 2010 <= num and num <= 2020

# four digits; at least 2020 and at most 2030.
def validateExpiration(value):
    if not re.match('^\d{4}$', value):
        return False
    num = int(value)
    return 2020 <= num and num <= 2030

# a number followed by either cm or in:
#   If cm, the number must be at least 150 and at most 193.
#   If in, the number must be at least 59 and at most 76.
def validateHeight(value):
    result = re.match('^(\d{1,})(cm|in)$', value)
    if not result:
        return False
    h = int(result[1])
    if result[2] == 'cm':
        return 150 <= h and h <= 193
    else:
        return 59 <= h and h <= 76

# a # followed by exactly six characters 0-9 or a-f.
def validteHairColor(value):
    return re.match('^#[0-9a-f]{6}$', value)

# exactly one of: amb blu brn gry grn hzl oth.
def validateEyeColor(value):
    return value in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth')

# a nine-digit number, including leading zeroes.
def validatePassportId(value):
    return re.match('^[0-9]{9}$', value)

validators = {
    'byr': validateBirthYear,
    'iyr': validateIssueYear,
    'eyr': validateExpiration,
    'hgt': validateHeight,
    'hcl': validteHairColor,
    'ecl': validateEyeColor,
    'pid': validatePassportId,
}

def isValidPassport(passport):
    num_valid_fields = 0
    for field in passport.split(' '):
        key, value = field.split(':')
        if key == 'cid':
            continue
        if not validators[key](value):
            return False
        num_valid_fields += 1
    return num_valid_fields == len(validators)

passports = ' '.join(open("input/04.txt").read().splitlines()).split('  ')

num_valid = 0
for passport in passports:
    if isValidPassport(passport):
        num_valid += 1
print(num_valid)

