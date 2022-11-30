from os.path import dirname


def find_loop():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')
        lines = lines[:-1]
        original_nth_changed = 0
        nth_changed = 0
        while True:
            broken = False
            i = 0
            accumulator = 0
            executed_instructions = [False] * len(lines)
            while i < len(lines):
                line = lines[i]
                if executed_instructions[i] is False:
                    executed_instructions[i] = True
                else:
                    broken = True
                    break
                (instruction, number) = line.split(' ')
                if instruction == "acc":
                    accumulator += int(number)
                    i += 1
                elif instruction == "jmp":
                    if nth_changed == 0:
                        i += 1
                    else:
                        i += int(number)
                    nth_changed -= 1
                elif instruction == "nop":
                    if nth_changed == 0:
                        i += int(number)
                    else:
                        i += 1
                    nth_changed -= 1
            if broken is False:
                return accumulator
            original_nth_changed += 1
            nth_changed = original_nth_changed


if __name__ == '__main__':
    print(find_loop())
