import string
from os.path import dirname


def find_your_seat():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        data = file.read()
        lines = data.split('\n')
        all_seats = list(range(8 * 128 - 1))
        for line in lines[:-1]:
            row = ''.join([(lambda a: '0' if a == 'F' else '1')(a) for a in line[:-3]])
            column = ''.join([(lambda a: '0' if a == 'L' else '1')(a) for a in line[-3:]])
            boarding_id = int(row, 2) * 8 + int(column, 2)
            all_seats[boarding_id] = ' '
        return ''.join(map(str, all_seats)).strip(string.digits).strip()


if __name__ == '__main__':
    print(find_your_seat())
