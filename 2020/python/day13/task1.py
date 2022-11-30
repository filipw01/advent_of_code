from os.path import dirname


def find_bus():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        (timestamp, busses) = file.read().split('\n')[:-1]
        timestamp = int(timestamp)
        busses = list(map(lambda bus: int(bus), filter(lambda bus: bus != 'x', busses.split(','))))
        minimum = {'id': busses[0], 'delay': busses[0]}
        for bus in busses:
            delay = bus - timestamp % bus
            if delay < minimum['delay']:
                minimum['delay'] = delay
                minimum['id'] = bus
        return minimum['delay'] * minimum['id']


if __name__ == '__main__':
    print(find_bus())
