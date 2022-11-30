from os.path import dirname


def check_passwords():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.readlines()
        valid_passwords = []
        for line in lines:
            [requirements, password] = line.split(': ')
            [required_count, required_letter] = requirements.split(' ')
            [first_index, second_index] = required_count.split('-')
            picked_letters = password[int(first_index) - 1] + password[int(second_index) - 1]
            letter_count = picked_letters.count(required_letter)
            if letter_count == 1:
                valid_passwords.append(password)
        return len(valid_passwords)


if __name__ == '__main__':
    print(check_passwords())
