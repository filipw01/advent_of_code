from os.path import dirname


def find_bus():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        data = file.read().split('\n')[1].split(',')
        busses = []
        # Use Chinese Reminder Theorem (all bus numbers are pairwise coprime)
        for (delay, bus) in enumerate(data):
            if bus == 'x':
                continue
            if delay % int(bus) == 0:
                busses.append([int(bus), 0])
            else:
                busses.append([int(bus), int(bus) - (delay % int(bus))])
        busses = sorted(busses, key=lambda x: x[0], reverse=True)
        index = 1
        multiplier = busses[0][0]
        prev = busses[0][1]
        while index < len(busses):
            prev = prev + multiplier
            if busses[index][1] == prev % busses[index][0]:
                multiplier *= busses[index][0]
                index += 1
        return prev


if __name__ == '__main__':
    print(find_bus())
