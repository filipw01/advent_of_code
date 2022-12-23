use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

enum Shape {
    Minus,
    Plus,
    VerticalLine,
    Block,
    ReverseL,
}

impl Shape {
    fn from_turn(turn: u8) -> Self {
        match turn % 5 {
            0 => Shape::Minus,
            1 => Shape::Plus,
            2 => Shape::ReverseL,
            3 => Shape::VerticalLine,
            4 => Shape::Block,
            _ => panic!("Invalid turn"),
        }
    }

    fn get_height(&self) -> u8 {
        match self {
            Shape::Minus => 1,
            Shape::Plus => 3,
            Shape::VerticalLine => 4,
            Shape::Block => 2,
            Shape::ReverseL => 3,
        }
    }
}

struct Rock {
    shape: Shape,
    x: u8,
    y: u8,
}

impl From<u8> for Rock {
    fn from(turn: u8) -> Self {
        let shape = Shape::from_turn(turn);
        Rock { shape, x: 2, y: 0 }
    }
}

impl Rock {
    fn is_touching_left(&self, grid: &VecDeque<[Cell; 7]>) -> bool {
        if self.x == 0 {
            return true;
        }
        let x = self.x as usize;
        let y = self.y as usize;
        match self.shape {
            Shape::Minus => grid[y][x - 1] == Cell::Full,
            Shape::Plus => {
                grid[y - 1][x - 1] == Cell::Full
                    || grid[y][x] == Cell::Full
                    || grid[y - 2][x] == Cell::Full
            }
            Shape::VerticalLine => {
                grid[y][x - 1] == Cell::Full
                    || grid[y - 1][x - 1] == Cell::Full
                    || grid[y - 2][x - 1] == Cell::Full
                    || grid[y - 3][x - 1] == Cell::Full
            }
            Shape::Block => grid[y][x - 1] == Cell::Full || grid[y - 1][x - 1] == Cell::Full,
            Shape::ReverseL => {
                grid[y][x + 1] == Cell::Full
                    || grid[y - 1][x + 1] == Cell::Full
                    || grid[y - 2][x - 1] == Cell::Full
            }
        }
    }

    fn is_touching_right(&self, grid: &VecDeque<[Cell; 7]>) -> bool {
        let x = self.x as usize;
        let y = self.y as usize;
        match self.shape {
            Shape::Minus => self.x == 3 || grid[y][x + 4] == Cell::Full,
            Shape::Plus => {
                self.x == 4
                    || grid[y][x + 2] == Cell::Full
                    || grid[y - 1][x + 3] == Cell::Full
                    || grid[y - 2][x + 2] == Cell::Full
            }
            Shape::VerticalLine => {
                self.x == 6
                    || grid[y][x + 1] == Cell::Full
                    || grid[y - 1][x + 1] == Cell::Full
                    || grid[y - 2][x + 1] == Cell::Full
                    || grid[y - 3][x + 1] == Cell::Full
            }
            Shape::Block => {
                self.x == 5 || grid[y][x + 2] == Cell::Full || grid[y - 1][x + 2] == Cell::Full
            }
            Shape::ReverseL => {
                self.x == 4
                    || grid[y][x + 3] == Cell::Full
                    || grid[y - 1][x + 3] == Cell::Full
                    || grid[y - 2][x + 3] == Cell::Full
            }
        }
    }

    fn is_touching_bottom(&self, grid: &VecDeque<[Cell; 7]>) -> bool {
        let x = self.x as usize;
        let y = self.y as usize;
        match self.shape {
            Shape::Minus => {
                self.y == 0
                    || grid[y - 1][x] == Cell::Full
                    || grid[y - 1][x + 1] == Cell::Full
                    || grid[y - 1][x + 2] == Cell::Full
                    || grid[y - 1][x + 3] == Cell::Full
            }
            Shape::Plus => {
                self.y == 2
                    || grid[y - 2][x] == Cell::Full
                    || grid[y - 3][x + 1] == Cell::Full
                    || grid[y - 2][x + 2] == Cell::Full
            }
            Shape::VerticalLine => self.y == 3 || grid[y - 4][x] == Cell::Full,
            Shape::Block => {
                self.y == 1 || grid[y - 2][x] == Cell::Full || grid[y - 2][x + 1] == Cell::Full
            }

            Shape::ReverseL => {
                self.y == 2
                    || grid[y - 3][x] == Cell::Full
                    || grid[y - 3][x + 1] == Cell::Full
                    || grid[y - 3][x + 2] == Cell::Full
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Full,
}

#[derive(Clone, Copy)]
enum Blow {
    Left,
    Right,
}

struct Cave {
    // rows from bottom to top each with 7 cells
    cells: VecDeque<[Cell; 7]>,
    rock_turn: usize,
    blow_turn: usize,
    height_offset: usize,
    blow_pattern: Vec<Blow>,
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self
            .cells
            .iter()
            .rev()
            .filter(|row| row.iter().any(|cell| *cell == Cell::Full))
        {
            for cell in row.iter() {
                match cell {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Full => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        Self {
            cells: VecDeque::new(),
            rock_turn: 0,
            blow_turn: 0,
            height_offset: 0,
            blow_pattern: s
                .trim()
                .chars()
                .map(|c| match c {
                    '<' => Blow::Left,
                    '>' => Blow::Right,
                    _ => panic!("Invalid blow pattern"),
                })
                .collect(),
        }
    }
}

impl Cave {
    fn get_height(&self) -> usize {
        let pos = self
            .cells
            .iter()
            .rposition(|row| row.iter().any(|cell| *cell == Cell::Full));
        if let Some(pos) = pos {
            pos + 1
        } else {
            0
        }
    }

    fn get_blow_direction(&self) -> Blow {
        self.blow_pattern[self.blow_turn % self.blow_pattern.len()]
    }

    fn render_rock(&mut self, rock: Rock) {
        let x = rock.x as usize;
        let y = rock.y as usize;
        match rock.shape {
            Shape::Minus => {
                self.cells[y][x] = Cell::Full;
                self.cells[y][x + 1] = Cell::Full;
                self.cells[y][x + 2] = Cell::Full;
                self.cells[y][x + 3] = Cell::Full;
            }
            Shape::Plus => {
                self.cells[y][x + 1] = Cell::Full;
                self.cells[y - 1][x] = Cell::Full;
                self.cells[y - 1][x + 1] = Cell::Full;
                self.cells[y - 1][x + 2] = Cell::Full;
                self.cells[y - 2][x + 1] = Cell::Full;
            }
            Shape::VerticalLine => {
                self.cells[y][x] = Cell::Full;
                self.cells[y - 1][x] = Cell::Full;
                self.cells[y - 2][x] = Cell::Full;
                self.cells[y - 3][x] = Cell::Full;
            }
            Shape::Block => {
                self.cells[y][x] = Cell::Full;
                self.cells[y][x + 1] = Cell::Full;
                self.cells[y - 1][x] = Cell::Full;
                self.cells[y - 1][x + 1] = Cell::Full;
            }
            Shape::ReverseL => {
                self.cells[y][x + 2] = Cell::Full;
                self.cells[y - 1][x + 2] = Cell::Full;
                self.cells[y - 2][x] = Cell::Full;
                self.cells[y - 2][x + 1] = Cell::Full;
                self.cells[y - 2][x + 2] = Cell::Full;
            }
        }
    }

    fn run_rock(&mut self) {
        let mut element = vec![];
        let warmup_rounds = 1000000;
        for index in 0..warmup_rounds {
            let mut rock = Rock::from((self.rock_turn % 5) as u8);

            let height = self.get_height();
            rock.y = height as u8 + 3 + rock.shape.get_height() - 1;
            if self.cells.len() <= rock.y as usize {
                let mut new_cells = VecDeque::from(vec![
                    [Cell::Empty; 7];
                    rock.y as usize - self.cells.len() + 5
                ]);
                self.cells.append(&mut new_cells);
            }

            loop {
                match self.get_blow_direction() {
                    Blow::Left => {
                        if !rock.is_touching_left(&self.cells) {
                            rock.x -= 1;
                        }
                    }
                    Blow::Right => {
                        if !rock.is_touching_right(&self.cells) {
                            rock.x += 1;
                        }
                    }
                }
                self.blow_turn += 1;
                if rock.is_touching_bottom(&self.cells) {
                    break;
                }
                rock.y -= 1;
            }
            self.render_rock(rock);
            if index > 30000 {
                element.push(self.get_height() - height);
            }
            self.rock_turn += 1;

            let mut occupied = [None; 7];
            for (row_index, row) in self.cells.iter().enumerate() {
                for (cell_index, cell) in row.iter().enumerate() {
                    if *cell == Cell::Full {
                        occupied[cell_index] = Some(row_index);
                    }
                }
            }
            if occupied.iter().all(|x| x.is_some()) {
                let min = occupied.iter().map(|x| x.unwrap()).min().unwrap();
                self.height_offset += min;
                for _ in 0..min {
                    self.cells.pop_front();
                }
            }
        }
        let cycle = find_cycle(&element);
        let left_to_next_cycle = cycle.len() - element.len() % cycle.len();

        // finish the last cycle
        self.height_offset += cycle.iter().rev().take(left_to_next_cycle).sum::<usize>();

        let cycle_height_change: usize = cycle.iter().sum();

        let iterations_left = 1_000_000_000_000_usize - warmup_rounds - left_to_next_cycle;

        // add the full cycles
        self.height_offset += (iterations_left / cycle.len()) * cycle_height_change;

        // add the remaining elements
        self.height_offset += cycle[0..(iterations_left % cycle.len())]
            .iter()
            .sum::<usize>();
    }
}
fn find_cycle(input: &Vec<usize>) -> Vec<usize> {
    let mut cycle = vec![];
    for index in 0..input.len() / 2 {
        cycle.push(input[index]);
        if cycle == input[cycle.len()..cycle.len() * 2].to_vec() {
            for check_index in 0..input.len() {
                if cycle[check_index % cycle.len()] != input[check_index] {
                    break;
                }
                if check_index == input.len() - 1 {
                    return cycle;
                }
            }
        }
    }
    vec![]
}

// Doesn't work for real input (probably because cycles are influenced by the rocks below)
pub fn solution(input: &str) -> usize {
    let mut cave = Cave::from(input);
    cave.run_rock();
    cave.get_height() + cave.height_offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"),
            1514285714288
        );
    }

    #[test]
    fn test_cycle() {
        assert_eq!(find_cycle(&vec![1, 2, 3, 1, 2, 3, 1, 2]), vec![1, 2, 3]);
        assert_eq!(
            find_cycle(&vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4,
                5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8,
                9, 10, 11,
            ]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            find_cycle(&vec![1, 2, 1, 2, 3, 1, 2, 1, 2, 3, 1, 2, 1, 2]),
            vec![1, 2, 1, 2, 3]
        );
        assert_eq!(find_cycle(&vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), vec![1, 2]);
        assert_eq!(
            find_cycle(&vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3]),
            Vec::<usize>::new()
        )
    }
}
