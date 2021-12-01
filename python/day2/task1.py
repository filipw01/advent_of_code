from os.path import dirname


def solve():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.readlines()

        return 'solution'


if __name__ == '__main__':
    print(solve())
