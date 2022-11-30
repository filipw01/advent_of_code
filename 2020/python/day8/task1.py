from os.path import dirname


def find_loop():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')
        accumulator = 0
        lines = lines[:-1]
        executed_instructions = [False] * len(lines)
        i = 0
        while True:
            line = lines[i]
            if executed_instructions[i] is False:
                executed_instructions[i] = True
            else:
                return accumulator
            (instruction, number) = line.split(' ')
            if instruction == "acc":
                accumulator += int(number)
                i += 1
            elif instruction == "jmp":
                i += int(number)
            else:
                i += 1


if __name__ == '__main__':
    print(find_loop())
