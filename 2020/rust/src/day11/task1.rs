use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<char>>,
    prev_rows: Option<Vec<Vec<char>>>,
}

impl From<Vec<&str>> for Board {
    fn from(lines: Vec<&str>) -> Self {
        Board {
            rows: lines.iter().map(|row| row.chars().collect()).collect(),
            prev_rows: None,
        }
    }
}

impl Board {
    fn run(&mut self) -> () {
        self.tick();
        loop {
            if self.check_if_changed(&self.rows) {
                self.tick();
            } else {
                return ();
            }
        }
    }

    fn tick(&mut self) {
        self.prev_rows = Some(self.rows.clone());
        for (row_index, row) in self.rows.iter_mut().enumerate() {
            for (cell_index, cell) in row.iter_mut().enumerate() {
                match cell {
                    'L' => {
                        if Board::count_neighbours(
                            &self.prev_rows.as_ref().unwrap(),
                            row_index,
                            cell_index,
                        ) == 0
                        {
                            *cell = '#'
                        }
                    }
                    '#' => {
                        if Board::count_neighbours(
                            &self.prev_rows.as_ref().unwrap(),
                            row_index,
                            cell_index,
                        ) > 3
                        {
                            *cell = 'L'
                        }
                    }
                    '.' => (),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn check_if_changed(&self, rows: &Vec<Vec<char>>) -> bool {
        for (index_row, row) in rows.iter().enumerate() {
            for (index_cell, _) in row.iter().enumerate() {
                if self.prev_rows.as_ref().unwrap()[index_row][index_cell]
                    != rows[index_row][index_cell]
                {
                    return true;
                }
            }
        }
        false
    }

    fn count_neighbours(prev_rows: &Vec<Vec<char>>, row_index: usize, cell_index: usize) -> usize {
        let mut neighbours = 0;
        let difference: [i32; 3] = [-1, 0, 1];
        for diff_row in difference.iter() {
            for diff_col in difference.iter() {
                let row = row_index as i32 + diff_row;
                let col = cell_index as i32 + diff_col;
                if row < 0 || col < 0 || (*diff_row == 0 && *diff_col == 0) {
                    continue;
                }
                let row = row as usize;
                let col = col as usize;
                if row >= prev_rows.len() || col >= prev_rows[row].len() {
                    continue;
                }
                if prev_rows[row][col] == '#' {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }
}

pub fn find_occupied_seats() -> usize {
    let file_content = read_to_string("day11/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut board = Board::from(lines);
    board.run();
    board
        .rows
        .iter()
        .map(|row| row.iter().filter(|char| **char == '#').count())
        .sum()
}
