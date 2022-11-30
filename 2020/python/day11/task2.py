from os.path import dirname


def find_occupied_seats():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')[:-1]
        board = Board(list(map(list, lines)))
        board.run()
        return sum(map(lambda row: len(list(filter(lambda cell: cell == "#", row))), board.rows))


class Board:
    def __init__(self, rows):
        self.rows = rows
        self.prev_rows = None

    def run(self):
        self.tick()
        while True:
            if self.check_if_changed(self.rows):
                self.tick()
            else:
                return None

    def tick(self):
        self.prev_rows = list(map(lambda row: row.copy(), self.rows))
        for row_index in range(len(self.rows)):
            for cell_index in range(len(self.rows[0])):
                if self.rows[row_index][cell_index] == "L":
                    if self.count_neighbours(row_index, cell_index) == 0:
                        self.rows[row_index][cell_index] = "#"
                elif self.rows[row_index][cell_index] == "#":
                    if self.count_neighbours(row_index, cell_index) > 4:
                        self.rows[row_index][cell_index] = "L"

    def count_neighbours(self, row_index, cell_index):
        neighbours = 0
        for diff_row in [-1, 0, 1]:
            for diff_col in [-1, 0, 1]:
                should_continue = False
                if diff_row == 0 and diff_col == 0:
                    continue
                multiplier = 0
                row = 0
                col = 0
                while multiplier == 0 or self.prev_rows[row][col] == ".":
                    multiplier += 1
                    row = row_index + multiplier * diff_row
                    col = cell_index + multiplier * diff_col
                    if row < 0 or col < 0 or row >= len(
                            self.prev_rows) or col >= len(self.prev_rows[row]):
                        should_continue = True
                        break
                if should_continue:
                    continue
                if self.prev_rows[row][col] == "#":
                    neighbours += 1
        return neighbours

    def check_if_changed(self, rows):
        for index_row in range(len(self.rows)):
            for index_cell in range(len(self.rows[index_row])):
                if self.prev_rows[index_row][index_cell] != rows[index_row][index_cell]:
                    return True
        return False


if __name__ == '__main__':
    print(find_occupied_seats())
