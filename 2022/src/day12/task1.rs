use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Error, Formatter};

const GREEN: &str = "\x1b[92m";
const RED: &str = "\x1b[91m";
const END_FMT: &str = "\x1b[0m";

struct Grid {
    grid: Vec<Vec<u8>>,
    current: (usize, usize),
    destination: (usize, usize),
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
        Grid {
            grid: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| match c {
                            'S' => {
                                current = (x, y);
                                b'a'
                            }
                            'E' => {
                                destination = (x, y);
                                b'z'
                            }
                            _ => c as u8,
                        })
                        .collect()
                })
                .collect(),
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
        if x > 0 && self.get(x - 1, y) <= value + 1 {
            steps.push((x - 1, y));
        }
        if x < self.grid[0].len() - 1 && self.get(x + 1, y) <= value + 1 {
            steps.push((x + 1, y));
        }
        if y > 0 && self.get(x, y - 1) <= value + 1 {
            steps.push((x, y - 1));
        }
        if y < self.grid.len() - 1 && self.get(x, y + 1) <= value + 1 {
            steps.push((x, y + 1));
        }
        steps
    }
}

struct PossiblePaths {
    to_visit: VecDeque<(usize, usize)>,
    distance: Vec<Vec<usize>>,
    visited: HashSet<(usize, usize)>,
}

impl From<&Grid> for PossiblePaths {
    fn from(grid: &Grid) -> Self {
        let height = grid.grid.len();
        let width = grid.grid[0].len();
        let mut distance = vec![vec![usize::MAX; width]; height];
        distance[grid.current.1][grid.current.0] = 0;
        PossiblePaths {
            to_visit: VecDeque::from([grid.current]),
            visited: HashSet::new(),
            distance,
        }
    }
}

impl PossiblePaths {
    fn get_distance(&self, to: (usize, usize)) -> usize {
        self.distance[to.1][to.0]
    }

    fn next(&mut self, grid: &Grid) {
        let last = self.to_visit.pop_front().unwrap();
        let possible = grid.get_next_possible_steps(last);
        if self.visited.contains(&last) {
            return;
        }
        // println!("last: {:?}", last);
        // println!("possible: {:?}", possible);
        let last_len = self.get_distance(last);
        self.visited.insert(last);
        possible
            .iter()
            .filter(|step| !self.visited.contains(step))
            .for_each(|step| {
                self.to_visit.push_back(*step);
                self.distance[step.1][step.0] = last_len + 1;
            });
        // println!("to visit: {:?}", self.to_visit);
    }
}

pub fn solution(input: &str) -> usize {
    let grid = Grid::from(input);
    let mut possible_paths = PossiblePaths::from(&grid);
    while !possible_paths.to_visit.is_empty() {
        // println!();
        // println!(
        //     "{}",
        //     possible_paths
        //         .distance
        //         .iter()
        //         .enumerate()
        //         .map(|(y, row)| row
        //             .iter()
        //             .enumerate()
        //             .map(|(x, num)| {
        //                 if *possible_paths.to_visit.iter().last().unwrap() == (x, y) {
        //                     return format!(
        //                         "{GREEN}{:02}{END_FMT}",
        //                         if num == &usize::MAX {
        //                             "#".to_string()
        //                         } else {
        //                             num.to_string()
        //                         }
        //                     );
        //                 }
        //                 format!(
        //                     "{:02}",
        //                     if num == &usize::MAX {
        //                         "#".to_string()
        //                     } else {
        //                         num.to_string()
        //                     }
        //                 )
        //             })
        //             .collect::<Vec<String>>()
        //             .join(" "))
        //         .collect::<Vec<String>>()
        //         .join("\n")
        // );
        possible_paths.next(&grid);
        println!("{}", possible_paths.to_visit.len());
    }
    possible_paths.get_distance(grid.destination)
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
            31
        );
    }
}
