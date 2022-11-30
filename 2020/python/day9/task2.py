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


def code_weakness(lines):
    i = 0
    while True:
        if not has_sum(lines[i:i + 25], lines[i + 25]):
            return int(lines[i + 25])
        i += 1


def sum_set(lines):
    result = 0
    for line in lines:
        result += int(line)
    return result


def break_code():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')
        weakness = code_weakness(lines)
        start = 0
        end = 1
        while True:
            continuous_sum = sum_set(lines[start:end])
            if continuous_sum > weakness:
                start += 1
            elif continuous_sum < weakness:
                end += 1
            elif continuous_sum == weakness:
                return min(map(lambda l: int(l), lines[start:end])) + max(map(lambda l: int(l), lines[start:end]))


if __name__ == '__main__':
    print(break_code())
