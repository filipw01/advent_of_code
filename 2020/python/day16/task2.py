from functools import reduce
from os.path import dirname
from collections import defaultdict


def run():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        (fields, my_ticket, nearby_tickets) = file.read().split('\n\n')
        labels = list(map(lambda field: field.split(': ')[0], fields.split('\n')))
        ranges = list(map(field_line_to_range, fields.split('\n')))
        possible = defaultdict(set)

        valid_tickets = list(map(lambda a: list(map(int, a.split(','))), nearby_tickets.split('\n')[1:-1]))
        valid_tickets = list(filter(lambda a: is_valid_ticket(a, ranges), valid_tickets))
        print(len(valid_tickets))
        possible = defaultdict(set)
        for i in range(len(ranges)):
            for (requirement_index, rang) in enumerate(ranges):
                if all(ticket[i] in rang for ticket in valid_tickets):
                    possible[requirement_index].add(i)

        sorted_possible = sorted(possible, key=lambda index: len(possible[index]))
        confirmed = {}
        for key in sorted_possible:
            for i in possible[key]:
                if i not in confirmed.values():
                    confirmed[key] = i

        departure_indices = list(filter(lambda x: x >= 0,
                                        map(lambda entry: entry[0] if 'departure' in entry[1] else -1,
                                            enumerate(labels))))
        result_indices = [confirmed[index] for index in departure_indices]
        result = 1
        for (index, field) in enumerate(my_ticket.split('\n')[1].split(',')):
            if index in result_indices:
                result *= int(field)
        return result


def is_valid_ticket(ticket, valid_numbers):
    valid_numbers = reduce(lambda acc, curr: acc + curr, valid_numbers, valid_numbers[0])
    for field in ticket:
        if field not in valid_numbers:
            return False
    return True


def field_line_to_range(line):
    range_1 = range(int(line.split(': ')[1].split(' or ')[0].split('-')[0]),
                    int(line.split(': ')[1].split(' or ')[0].split('-')[1]) + 1)
    range_2 = range(int(line.split(': ')[1].split(' or ')[1].split('-')[0]),
                    int(line.split(': ')[1].split(' or ')[1].split('-')[1]) + 1)
    return list(range_1) + list(range_2)


if __name__ == '__main__':
    print(run())
