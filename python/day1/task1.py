from os.path import dirname


def calculate_depth_increase():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        depths = file.readlines()
        depth_increase_count = 0
        prev_depth = int(depths[0])
        for depth in depths:
            depth = int(depth)
            if depth > prev_depth:
                depth_increase_count += 1
            prev_depth = depth
        return depth_increase_count


if __name__ == '__main__':
    print(calculate_depth_increase())
