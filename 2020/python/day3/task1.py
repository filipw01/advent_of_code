from os.path import dirname


def calculate_encounters():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.readlines()
        x = 0
        trees = 0
        for line in lines:
            line = line[:-1]
            cell_index = x % len(line)
            cell = line[cell_index]
            if cell == "#":
                trees += 1
            x += 3
        return trees


if __name__ == '__main__':
    print(calculate_encounters())
