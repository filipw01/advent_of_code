use std::fmt::{Debug, Formatter};

enum Shape {
    Minus,
    Plus,
    VerticalLine,
    Block,
    ReverseL,
}

impl Shape {
    fn from_turn(turn: usize) -> Self {
        match turn % 5 {
            0 => Shape::Minus,
            1 => Shape::Plus,
            2 => Shape::ReverseL,
            3 => Shape::VerticalLine,
            4 => Shape::Block,
            _ => panic!("Invalid turn"),
        }
    }

    fn get_height(&self) -> usize {
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
    x: usize,
    y: usize,
}

impl From<usize> for Rock {
    fn from(turn: usize) -> Self {
        let shape = Shape::from_turn(turn);
        Rock { shape, x: 2, y: 0 }
    }
}

impl Rock {
    fn is_touching_left(&self, grid: &Vec<[Cell; 7]>) -> bool {
        if self.x == 0 {
            return true;
        }
        match self.shape {
            Shape::Minus => *grid[self.y].get(self.x - 1).unwrap() == Cell::Full,
            Shape::Plus => {
                *grid.get(self.y - 1).unwrap().get(self.x - 1).unwrap() == Cell::Full
                    || grid[self.y][self.x] == Cell::Full
                    || grid.get(self.y - 2).unwrap()[self.x] == Cell::Full
            }
            Shape::VerticalLine => {
                *grid[self.y].get(self.x - 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x - 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 2).unwrap().get(self.x - 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 3).unwrap().get(self.x - 1).unwrap() == Cell::Full
            }
            Shape::Block => {
                *grid[self.y].get(self.x - 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x - 1).unwrap() == Cell::Full
            }
            Shape::ReverseL => {
                *grid[self.y].get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 2).unwrap().get(self.x - 1).unwrap() == Cell::Full
            }
        }
    }

    fn is_touching_right(&self, grid: &Vec<[Cell; 7]>) -> bool {
        match self.shape {
            Shape::Minus => self.x == 3 || *grid[self.y].get(self.x + 4).unwrap() == Cell::Full,
            Shape::Plus => {
                self.x == 4
                    || *grid[self.y].get(self.x + 2).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 3).unwrap() == Cell::Full
                    || *grid.get(self.y - 2).unwrap().get(self.x + 2).unwrap() == Cell::Full
            }
            Shape::VerticalLine => {
                self.x == 6
                    || *grid[self.y].get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 2).unwrap().get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 3).unwrap().get(self.x + 1).unwrap() == Cell::Full
            }
            Shape::Block => {
                self.x == 5
                    || *grid[self.y].get(self.x + 2).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 2).unwrap() == Cell::Full
            }
            Shape::ReverseL => {
                self.x == 4
                    || *grid[self.y].get(self.x + 3).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 3).unwrap() == Cell::Full
                    || *grid.get(self.y - 2).unwrap().get(self.x + 3).unwrap() == Cell::Full
            }
        }
    }

    fn is_touching_bottom(&self, grid: &Vec<[Cell; 7]>) -> bool {
        match self.shape {
            Shape::Minus => {
                self.y == 0
                    || *grid.get(self.y - 1).unwrap().get(self.x).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 2).unwrap() == Cell::Full
                    || *grid.get(self.y - 1).unwrap().get(self.x + 3).unwrap() == Cell::Full
            }
            Shape::Plus => {
                self.y == 2
                    || *grid.get(self.y - 2).unwrap().get(self.x).unwrap() == Cell::Full
                    || *grid.get(self.y - 3).unwrap().get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 2).unwrap().get(self.x + 2).unwrap() == Cell::Full
            }
            Shape::VerticalLine => {
                self.y == 3 || *grid.get(self.y - 4).unwrap().get(self.x).unwrap() == Cell::Full
            }
            Shape::Block => {
                self.y == 1
                    || *grid.get(self.y - 2).unwrap().get(self.x).unwrap() == Cell::Full
                    || *grid.get(self.y - 2).unwrap().get(self.x + 1).unwrap() == Cell::Full
            }

            Shape::ReverseL => {
                self.y == 2
                    || *grid.get(self.y - 3).unwrap().get(self.x).unwrap() == Cell::Full
                    || *grid.get(self.y - 3).unwrap().get(self.x + 1).unwrap() == Cell::Full
                    || *grid.get(self.y - 3).unwrap().get(self.x + 2).unwrap() == Cell::Full
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
    cells: Vec<[Cell; 7]>,
    rock_turn: usize,
    blow_turn: usize,
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
            cells: vec![[Cell::Empty; 7]; 4],
            rock_turn: 0,
            blow_turn: 0,
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
        match rock.shape {
            Shape::Minus => {
                self.cells[rock.y][rock.x] = Cell::Full;
                self.cells[rock.y][rock.x + 1] = Cell::Full;
                self.cells[rock.y][rock.x + 2] = Cell::Full;
                self.cells[rock.y][rock.x + 3] = Cell::Full;
            }
            Shape::Plus => {
                self.cells[rock.y][rock.x + 1] = Cell::Full;
                self.cells[rock.y - 1][rock.x] = Cell::Full;
                self.cells[rock.y - 1][rock.x + 1] = Cell::Full;
                self.cells[rock.y - 1][rock.x + 2] = Cell::Full;
                self.cells[rock.y - 2][rock.x + 1] = Cell::Full;
            }
            Shape::VerticalLine => {
                self.cells[rock.y][rock.x] = Cell::Full;
                self.cells[rock.y - 1][rock.x] = Cell::Full;
                self.cells[rock.y - 2][rock.x] = Cell::Full;
                self.cells[rock.y - 3][rock.x] = Cell::Full;
            }
            Shape::Block => {
                self.cells[rock.y][rock.x] = Cell::Full;
                self.cells[rock.y][rock.x + 1] = Cell::Full;
                self.cells[rock.y - 1][rock.x] = Cell::Full;
                self.cells[rock.y - 1][rock.x + 1] = Cell::Full;
            }
            Shape::ReverseL => {
                self.cells[rock.y][rock.x + 2] = Cell::Full;
                self.cells[rock.y - 1][rock.x + 2] = Cell::Full;
                self.cells[rock.y - 2][rock.x] = Cell::Full;
                self.cells[rock.y - 2][rock.x + 1] = Cell::Full;
                self.cells[rock.y - 2][rock.x + 2] = Cell::Full;
            }
        }
    }

    fn run_rock(&mut self) {
        // add new rock

        let mut rock = Rock::from(self.rock_turn);
        rock.y = self.get_height() + 3 + rock.shape.get_height() - 1;

        self.cells
            .append(&mut vec![[Cell::Empty; 7]; rock.y - self.get_height()]);

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
        self.rock_turn += 1;
    }
}

pub fn solution(input: &str) -> usize {
    let mut cave = Cave::from(input);
    for _rock in 0..2022 {
        cave.run_rock();
    }
    cave.get_height()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"), 3068);
    }
}
