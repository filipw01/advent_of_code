from os.path import dirname


def get_gamma(gamma_list):
    length = 12
    gamma = [0 for _ in range(length)]
    for line in gamma_list:
        for i in range(length):
            match line[i]:
                case '0':
                    gamma[i] -= 1
                case '1':
                    gamma[i] += 1
    return [0 if bit < 0 else 1 for bit in gamma]


def get_oxygen(oxygen):
    index = 0
    while True:
        gamma = get_gamma(oxygen)
        if len(oxygen) == 1:
            return int(oxygen[0], 2)
        oxygen = [x for x in oxygen if int(x[index]) == gamma[index]]
        index += 1


def get_co2(co2):
    index = 0
    while True:
        gamma = get_gamma(co2)
        if len(co2) == 1:
            return int(co2[0], 2)
        co2 = [x for x in co2 if int(x[index]) == (gamma[index] - 1) * -1]
        index += 1


def solve():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        data = list(file)
        return get_oxygen(data) * get_co2(data)


if __name__ == '__main__':
    print(solve())
