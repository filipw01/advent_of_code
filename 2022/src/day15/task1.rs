use itertools::Itertools;
use std::fmt::{Debug, Display};

struct Row {
    cells: Vec<Cell>,
}

impl From<(&str, i32)> for Row {
    fn from((s, row): (&str, i32)) -> Self {
        let sensors_and_beacons: Vec<((i32, i32), (i32, i32))> = s
            .trim()
            .lines()
            .map(|line| {
                let (sensor_raw, beacon_raw) =
                    line.split_once(": closest beacon is at x=").unwrap();
                let sensor = sensor_raw
                    .trim_start_matches("Sensor at x=")
                    .split(", y=")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                let beacon = beacon_raw
                    .split(", y=")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (sensor, beacon)
            })
            .collect();
        let (min_x, max_x) = sensors_and_beacons
            .iter()
            .fold((i32::MAX, i32::MIN), |(min_x, max_x), ((x, _), (bx, _))| {
                (min_x.min(*x).min(*bx), max_x.max(*x).max(*bx))
            });
        let max_distance = sensors_and_beacons
            .iter()
            .map(|((x, y), (bx, by))| (x - bx).abs() + (y - by).abs())
            .max()
            .unwrap();
        let mut cells = vec![Cell::Empty; (max_x - min_x + 1 + (max_distance * 2)) as usize];
        let x_offset = min_x - max_distance;
        let width = cells.len();
        sensors_and_beacons.iter().for_each(|((x, y), (bx, by))| {
            let row_y = row;
            let coverage_range = (bx - x).abs() + (by - y).abs();
            let row_distance = (row_y - y).abs();
            if row_distance > coverage_range {
                return;
            }
            if row_y == *y {
                cells[(x - x_offset) as usize] = Cell::Sensor;
            }
            if row_y == *by {
                cells[(bx - x_offset) as usize] = Cell::Beacon;
            }
            (0..=(coverage_range - row_distance)).for_each(|i| {
                let left = x - x_offset - i;
                let right = x - x_offset + i;
                assert!(left >= 0);
                assert!(right < width as i32);
                if cells[left as usize] == Cell::Empty {
                    cells[left as usize] = Cell::Covered;
                };
                if cells[right as usize] == Cell::Empty {
                    cells[right as usize] = Cell::Covered;
                };
            });
        });

        Row { cells }
    }
}

impl Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    Beacon,
    Sensor,
    Empty,
    Covered,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Beacon => write!(f, "B"),
            Cell::Sensor => write!(f, "S"),
            Cell::Empty => write!(f, "."),
            Cell::Covered => write!(f, "#"),
        }
    }
}

pub fn solution(input: &str, row: i32) -> usize {
    let cells = Row::from((input, row));

    cells
        .cells
        .iter()
        .filter(|cell| **cell == Cell::Covered)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
                10
            ),
            26
        );
    }
}
