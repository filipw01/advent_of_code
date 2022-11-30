from os.path import dirname


def has_sum(numbers, solution):
    for (index_1, line_1) in enumerate(numbers):
        line_1 = int(line_1)
        for (index_2, line_2) in enumerate(numbers):
            line_2 = int(line_2)
            if index_1 == index_2:
                continue
            if line_1 + line_2 == int(solution):
                return True
    return False


def code_weakness():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')
        i = 0
        while True:
            if not has_sum(lines[i:i + 25], lines[i + 25]):
                return lines[i + 25]
            i += 1


if __name__ == '__main__':
    print(code_weakness())
