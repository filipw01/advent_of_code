from os.path import dirname


def solve():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        (x, z) = (0, 0)
        for line in file:
            match line.split(' '):
                case ['forward', moveX]:
                    x += int(moveX)
                case ['down', moveZ]:
                    z += int(moveZ)
                case ['up', moveZ]:
                    z -= int(moveZ)

        return x * z


if __name__ == '__main__':
    print(solve())
