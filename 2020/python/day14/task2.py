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
                for masked_address in calculate_addresses(address, mask):
                    memory[masked_address] = int(value)
        return sum(memory.values())


def calculate_addresses(address, mask):
    value = str(bin(int(address))[2:])[::-1]
    mask = mask[::-1]
    address = ''
    for index in range(len(mask)):
        if mask[index] == '0':
            if index < len(value):
                address += value[index]
            else:
                address += '0'
        else:
            address += mask[index]
    address = address[::-1]
    possible_addresses = []
    x_count = address.count('X')
    values_to_insert = ['1' for _ in range(x_count)]
    while True:
        temp_address = address
        for bit in values_to_insert:
            temp_address = temp_address.replace('X', bit, 1)
        possible_addresses.append(temp_address)
        if int(''.join(values_to_insert), 2) == 0:
            return possible_addresses
        values_to_insert = [bit for bit in prepend_while('0', bin(int(''.join(values_to_insert), 2) - 1)[2:], x_count)]


def prepend_while(value, text, length):
    while len(text) < length:
        text = value + text
    return text


if __name__ == '__main__':
    print(emulate())
