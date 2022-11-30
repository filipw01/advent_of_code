from os.path import dirname


def find_way():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')[:-1]
        x = 10
        y = 1
        ship_x = 0
        ship_y = 0
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
                if value == 90:
                    x, y = -y, x
                elif value == 180:
                    x, y = -x, -y
                elif value == 270:
                    x, y = y, -x
            elif instruction == "R":
                if value == 90:
                    x, y = y, -x
                elif value == 180:
                    x, y = -x, -y
                elif value == 270:
                    x, y = -y, x
            elif instruction == "F":
                ship_x += value * x
                ship_y += value * y
        return abs(ship_x) + abs(ship_y)


if __name__ == '__main__':
    print(find_way())
