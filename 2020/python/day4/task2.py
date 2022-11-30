import string
from os.path import dirname


def validate_passport(passport):
    required_keys = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
    passport_keys = {(lambda field: field.split(":")[0])(field) for field in passport}
    if len(required_keys - passport_keys) != 0:
        return False
    for field in passport:
        key, value = field.split(":")
        if key == 'byr' and (1920 > int(value) or 2002 < int(value)):
            return False
        if key == 'iyr' and (2010 > int(value) or 2020 < int(value)):
            return False
        if key == 'eyr' and (2020 > int(value) or 2030 < int(value)):
            return False
        if key == 'hgt':
            if value[-2:] in ['cm', 'in']:
                height = int(value[:-2])
                unit = value[-2:]
                if unit == 'cm' and (150 > height or height > 193):
                    return False
                if unit == 'in' and (59 > height or height > 76):
                    return False
            else:
                return False
        if key == 'hcl' and (value[0] != '#' or len(value) != 7 or any(
                letter not in string.hexdigits for letter in value[1:])):
            return False
        if key == 'ecl' and value not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
            return False
        if key == 'pid' and (len(value) != 9 or any(digit not in string.digits for digit in value)):
            return False
    return True


def count_valid_passports():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        file = file.read()
        possible_passports = file.split('\n\n')
        valid_passports = 0
        for possible_passport in possible_passports:
            messy_passport = map(lambda passport_part: passport_part.split(' '), possible_passport.split('\n'))
            flat_passport = {field for messy_fields in messy_passport for field in messy_fields if ':' in field}
            if validate_passport(flat_passport):
                valid_passports += 1
        return valid_passports


if __name__ == '__main__':
    print(count_valid_passports())
