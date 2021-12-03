from os.path import dirname


def solve():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        gamma = [0 for _ in range(12)]

        for line in file:
            for i in range(12):
                match line[i]:
                    case '0':
                        gamma[i] -= 1
                    case '1':
                        gamma[i] += 1
        gamma = [0 if bit < 0 else 1 for bit in gamma]
        print(gamma)
        gamma_bin = ''.join([str(bit) for bit in gamma])
        epsilon_bin = ''.join([str((bit - 1) * -1) for bit in gamma])
        gamma_num = int(gamma_bin, 2)
        epsilon_num = int(epsilon_bin, 2)

        return gamma_num * epsilon_num


if __name__ == '__main__':
    print(solve())
