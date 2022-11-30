from functools import reduce
from os.path import dirname


def calculate_encounters(right_move, down_move):
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.readlines()
        x = 0
        trees = 0
        for line in [line for (index, line) in enumerate(lines) if index % down_move == 0]:
            line = line[:-1]
            cell_index = x % len(line)
            cell = line[cell_index]
            if cell == "#":
                trees += 1
            x += right_move
        return trees


def calculate_solution():
    return reduce(lambda acc, curr: acc * calculate_encounters(curr[0], curr[1]),
                  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)], 1)


if __name__ == '__main__':
    print(calculate_solution())
