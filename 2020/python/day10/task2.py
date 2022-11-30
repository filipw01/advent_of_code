from collections import defaultdict
from os.path import dirname


def find_valid_combinations():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        adapters = sorted(map(int, file.read().split('\n')[:-1]))
        max_adapter = max(adapters)
        results = defaultdict(lambda: 0)
        results[0] += 1
        for adapter in adapters:
            temp_results = results.copy()
            for result, count in results.items():
                if result + 1 == adapter or result + 2 == adapter or result + 3 == adapter:
                    temp_results[adapter] += temp_results[result]
            results = temp_results
        return results[max_adapter]


if __name__ == '__main__':
    print(find_valid_combinations())
