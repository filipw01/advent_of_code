from os.path import dirname


def emulate():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')[:-1]
        mask = ''
        memory = {}
        for line in lines:
            (operation, value) = line.split(' = ')
            if operation == 'mask':
                mask = value
            if operation[:3] == 'mem':
                address = operation[4:-1]
                memory[address] = apply_mask(value, mask)
        return sum(memory.values())


def apply_mask(value, mask):
    value = str(bin(int(value))[2:])[::-1]
    mask = mask[::-1]
    output_number = ''
    for index in range(len(mask)):
        if mask[index] == 'X':
            if index < len(value):
                output_number += value[index]
            else:
                output_number += '0'
        else:
            output_number += mask[index]
    return int(output_number[::-1], 2)


if __name__ == '__main__':
    print(emulate())
