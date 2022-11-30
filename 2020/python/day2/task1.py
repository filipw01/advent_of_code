from os.path import dirname


def check_passwords():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.readlines()
        valid_passwords = []
        for line in lines:
            [requirements, password] = line.split(': ')
            [required_count, required_letter] = requirements.split(' ')
            [min_count, max_count] = required_count.split('-')
            letter_count = password.count(required_letter)
            if int(min_count) <= letter_count <= int(max_count):
                valid_passwords.append(password)
        return len(valid_passwords)


if __name__ == '__main__':
    print(check_passwords())
