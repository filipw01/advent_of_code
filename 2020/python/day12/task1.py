from os.path import dirname


def find_way():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')[:-1]
        x = 0
        y = 0
        rotation = 90
        for line in lines:
            instruction = line[0]
            value = int(line[1:])
            if instruction == "N":
                y += value
            elif instruction == "S":
                y -= value
            elif instruction == "E":
                x += value
            elif instruction == "W":
                x -= value
            elif instruction == "L":
                rotation -= value
            elif instruction == "R":
                rotation += value
            elif instruction == "F":
                if rotation % 360 == 0:
                    y += value
                elif rotation % 360 == 90:
                    x += value
                elif rotation % 360 == 180:
                    y -= value
                elif rotation % 360 == 270:
                    x -= value
        return abs(x) + abs(y)


if __name__ == '__main__':
    print(find_way())
