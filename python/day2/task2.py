from os.path import dirname


def solve():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        (x, z, aim) = (0, 0, 0)
        for line in file:
            match line.split(' '):
                case ['forward', moveX]:
                    x += int(moveX)
                    z += int(moveX) * aim
                case ['down', moveAim]:
                    aim += int(moveAim)
                case ['up', moveAim]:
                    aim -= int(moveAim)

        return x * z


if __name__ == '__main__':
    print(solve())
