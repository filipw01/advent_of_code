from os.path import dirname


def calculate_depth_increase():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        depths = list(map(int, file.readlines()))
        depth_increase_count = 0
        prev_depth = depths[0] + depths[1] + depths[2]
        for (index, depth) in enumerate(depths[1:-2]):
            depth = depths[index+1] + depths[index + 2] + depths[index + 3]
            if depth > prev_depth:
                depth_increase_count += 1
            prev_depth = depth
        return depth_increase_count


if __name__ == '__main__':
    print(calculate_depth_increase())
