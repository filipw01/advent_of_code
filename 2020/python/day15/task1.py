from os.path import dirname


def play():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        numbers = list(map(int, file.read().split('\n')[0].split(',')))
        memory = {}
        last_number = numbers[len(numbers) - 1]
        turn = 1
        for index in range(len(numbers) - 1):
            memory[numbers[index]] = turn
            turn += 1
        while True:
            if last_number in memory:
                last_occurrence = memory[last_number]
                memory[last_number] = turn
                last_number = turn - last_occurrence
            else:
                memory[last_number] = turn
                last_number = 0
            turn += 1
            if turn == 2020:
                return last_number


if __name__ == '__main__':
    print(play())
