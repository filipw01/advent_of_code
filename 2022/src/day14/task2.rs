use itertools::Itertools;
use std::fmt::{Debug, Display};

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new((_, max_x, _, max_y): (usize, usize, usize, usize)) -> Self {
        let mut cells = vec![vec![Cell::Air; max_x * 2]; max_y + 3];
        let grid_last_row = cells.len() - 1;
        cells[grid_last_row] = cells[grid_last_row].iter().map(|_| Cell::Rock).collect();
        Grid { cells }
    }

    fn get(&self, (x, y): (usize, usize)) -> &Cell {
        &self.cells[y][x]
    }

    fn add_rock(&mut self, rock: &Rock) {
        let mut previous_instruction = rock.instructions.first().unwrap();
        for instruction in rock.instructions.iter() {
            if previous_instruction == instruction {
                continue;
            }
            let y_range = if previous_instruction.1 < instruction.1 {
                previous_instruction.1..=instruction.1
            } else {
                instruction.1..=previous_instruction.1
            };
            if previous_instruction.0 == instruction.0 {
                y_range.for_each(|y| self.cells[y][instruction.0] = Cell::Rock);
            }

            let x_range = if previous_instruction.0 < instruction.0 {
                previous_instruction.0..=instruction.0
            } else {
                instruction.0..=previous_instruction.0
            };
            if previous_instruction.1 == instruction.1 {
                x_range.for_each(|x| self.cells[instruction.1][x] = Cell::Rock);
            }

            previous_instruction = instruction
        }
    }

    fn add_new_sand(&mut self) -> Option<(usize, usize)> {
        if self.cells[0][500] == Cell::Sand {
            return None;
        }
        self.cells[0][500] = Cell::Sand;
        Some((500, 0))
    }

    fn move_sand(&mut self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self.get((x, y + 1)) {
            Cell::Air => {
                self.cells[y][x] = Cell::Air;
                self.cells[y + 1][x] = Cell::Sand;
                Some((x, y + 1))
            }
            _ => {
                let left_diagonal = self.get((x - 1, y + 1));
                match left_diagonal {
                    Cell::Air => {
                        self.cells[y][x] = Cell::Air;
                        self.cells[y + 1][x - 1] = Cell::Sand;
                        Some((x - 1, y + 1))
                    }
                    _ => {
                        let right_diagonal = self.get((x + 1, y + 1));
                        match right_diagonal {
                            Cell::Air => {
                                self.cells[y][x] = Cell::Air;
                                self.cells[y + 1][x + 1] = Cell::Sand;
                                Some((x + 1, y + 1))
                            }
                            _ => None,
                        }
                    }
                }
            }
        }
    }

    fn run(&mut self) -> usize {
        let mut last_sand = self.add_new_sand();
        let mut sand_count: usize = 1;
        loop {
            if sand_count % 1000 == 0 {
                println!("Sand count: {}", sand_count);
            }
            // println!("{:?}", self);
            if let Some((x, y)) = last_sand {
                last_sand = self.move_sand((x, y));
            } else {
                last_sand = self.add_new_sand();
                if last_sand.is_none() {
                    return sand_count;
                }
                sand_count += 1;
            }
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Air => write!(f, "."),
            Cell::Rock => write!(f, "#"),
            Cell::Sand => write!(f, "|"),
        }
    }
}

#[derive(Debug)]
struct Rock {
    instructions: Vec<(usize, usize)>,
}

impl From<&str> for Rock {
    fn from(s: &str) -> Self {
        Rock {
            instructions: s
                .split(" -> ")
                .map(|instruction| {
                    let (x, y) = instruction.split(',').collect_tuple().unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect(),
        }
    }
}

impl Rock {
    fn get_boundary(&self) -> (usize, usize, usize, usize) {
        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;
        for instruction in self.instructions.iter() {
            min_x = min_x.min(instruction.0);
            max_x = max_x.max(instruction.0);
            min_y = min_y.min(instruction.1);
            max_y = max_y.max(instruction.1);
        }
        (min_x, max_x, min_y, max_y)
    }
}

pub fn solution(input: &str) -> usize {
    let rocks: Vec<_> = input.lines().map(Rock::from).collect();
    let boundary = rocks.iter().fold(
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        |acc, rock| {
            let (min_x, max_x, min_y, max_y) = rock.get_boundary();
            (
                acc.0.min(min_x),
                acc.1.max(max_x),
                acc.2.min(min_y),
                acc.3.max(max_y),
            )
        },
    );
    let mut grid = Grid::new(boundary);
    rocks.iter().for_each(|rock| grid.add_rock(rock));
    grid.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"
            ),
            93
        );
    }
}
