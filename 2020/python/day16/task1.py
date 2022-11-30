from os.path import dirname


def run():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        (fields, _, nearby_tickets) = file.read().split('\n\n')
    fields = list(map(field_line_to_ranges, fields.split('\n')))
    valid_numbers = set()
    for field in fields:
        for rang in field:
            for number in range(rang[0], rang[1]):
                valid_numbers.add(number)

    error_rate = 0
    for ticket in nearby_tickets.split('\n')[1:-1]:
        for field in ticket.split(','):
            field = int(field)
            if field not in valid_numbers:
                error_rate += field
    return error_rate


def field_line_to_ranges(line):
    return tuple(map(lambda r: tuple(map(int, r.split('-'))), line.split(': ')[1].split(' or ')))


if __name__ == '__main__':
    print(run())
