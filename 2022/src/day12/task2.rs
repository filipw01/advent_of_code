use std::fmt::{Debug, Error, Formatter};

const GREEN: &str = "\x1b[92m";
const RED: &str = "\x1b[91m";
const END_FMT: &str = "\x1b[0m";

struct Grid {
    grid: Vec<Vec<u8>>,
    current: (usize, usize),
    destination: (usize, usize),
    lowest_points: Vec<(usize, usize)>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for (y, row) in self.grid.iter().enumerate() {
            writeln!(
                f,
                "{}",
                row.iter()
                    .enumerate()
                    .map(|(x, num)| {
                        if (x, y) == self.current {
                            format!("{}{}{}", GREEN, *num as char, END_FMT)
                        } else if (x, y) == self.destination {
                            format!("{}{}{}", RED, *num as char, END_FMT)
                        } else {
                            format!("{}", *num as char)
                        }
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut current = (0, 0);
        let mut destination = (0, 0);
        let mut lowest_points = vec![];
        Grid {
            grid: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| match c {
                            'S' => {
                                lowest_points.push((x, y));
                                destination = (x, y);
                                b'a'
                            }
                            'E' => {
                                current = (x, y);
                                b'z'
                            }
                            'a' => {
                                lowest_points.push((x, y));
                                c as u8
                            }
                            _ => c as u8,
                        })
                        .collect()
                })
                .collect(),
            lowest_points,
            current,
            destination,
        }
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y][x]
    }

    fn get_next_possible_steps(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut steps = Vec::new();
        let value = self.get(x, y);
        if x > 0 && self.get(x - 1, y) >= value - 1 {
            steps.push((x - 1, y));
        }
        if x < self.grid[0].len() - 1 && self.get(x + 1, y) >= value - 1 {
            steps.push((x + 1, y));
        }
        if y > 0 && self.get(x, y - 1) >= value - 1 {
            steps.push((x, y - 1));
        }
        if y < self.grid.len() - 1 && self.get(x, y + 1) >= value - 1 {
            steps.push((x, y + 1));
        }
        steps
    }
}

struct PossiblePaths {
    visited: Vec<(usize, usize)>,
    distance: Vec<Vec<usize>>,
}

impl From<&Grid> for PossiblePaths {
    fn from(grid: &Grid) -> Self {
        let height = grid.grid.len();
        let width = grid.grid[0].len();
        let mut distance = vec![vec![usize::MAX; width]; height];
        distance[grid.current.1][grid.current.0] = 0;
        PossiblePaths {
            visited: vec![grid.current],
            distance,
        }
    }
}

impl PossiblePaths {
    fn get_distance(&self, to: (usize, usize)) -> usize {
        self.distance[to.1][to.0]
    }

    fn next(&mut self, grid: &Grid) {
        let last = self.visited.pop().unwrap();
        let possible = grid.get_next_possible_steps(last);

        // println!("last: {:?}", last);
        // println!("possible: {:?}", possible);
        let last_len = self.get_distance(last);
        possible.iter().for_each(|step| {
            let best_distance = self.get_distance(*step);
            if best_distance > last_len {
                self.visited = self
                    .visited
                    .clone()
                    .into_iter()
                    .filter(|v| step != v || self.get_distance(*v) == last_len + 1)
                    .collect()
            }
            if best_distance > last_len + 1 {
                self.visited.push(*step);
                self.distance[step.1][step.0] = last_len + 1;
            }
        });
        // println!("visited: {:?}", self.visited);
    }
}

pub fn solution(input: &str) -> usize {
    let grid = Grid::from(input);
    let mut possible_paths = PossiblePaths::from(&grid);
    while !possible_paths.visited.is_empty() {
        // for _ in 0..15 {
        // println!();

        possible_paths.next(&grid);
        println!("{}", possible_paths.visited.len());
    }
    println!(
        "{}",
        possible_paths
            .distance
            .iter()
            .enumerate()
            .map(|(_y, row)| row
                .iter()
                .enumerate()
                .map(|(_x, num)| {
                    format!(
                        "{:01}",
                        if num == &usize::MAX {
                            "#".to_string()
                        } else {
                            num.to_string()
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    );
    println!("{:?}", grid);
    grid.lowest_points
        .iter()
        .map(|point| possible_paths.get_distance(*point))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"
            ),
            29
        );
    }
}
