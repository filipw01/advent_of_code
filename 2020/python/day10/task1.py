from os.path import dirname


def calculate_adapters():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        adapters = set(map(int, file.read().split('\n')[:-1]))
        current_adapter = 0
        diff_1 = 0
        diff_3 = 1
        while True:
            if current_adapter + 1 in adapters:
                diff_1 += 1
                current_adapter += 1
            elif current_adapter + 2 in adapters:
                current_adapter += 2
            elif current_adapter + 3 in adapters:
                diff_3 += 1
                current_adapter += 3
            else:
                return diff_1 * diff_3


if __name__ == '__main__':
    print(calculate_adapters())
