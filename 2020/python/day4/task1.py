from os.path import dirname


def count_valid_passports():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        file = file.read()
        possible_passports = file.split('\n\n')
        required_keys = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
        valid_passports = 0
        for possible_passport in possible_passports:
            messy_passport = map(lambda passport_part: passport_part.split(' '), possible_passport.split('\n'))
            flat_passport = {field for messy_fields in messy_passport for field in messy_fields if ':' in field}
            passport_keys = {(lambda field: field.split(":")[0])(field) for field in flat_passport}
            if len(required_keys - passport_keys) == 0:
                valid_passports += 1
        return valid_passports


if __name__ == '__main__':
    print(count_valid_passports())
