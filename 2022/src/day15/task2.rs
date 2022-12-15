use itertools::Itertools;
use std::fmt::{Debug, Display};

struct Row {
    solution: Option<usize>,
}

struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}

struct Sensors {
    sensors: Vec<Sensor>,
}

impl Sensors {
    fn is_covered(&self, x: i32, y: i32) -> bool {
        self.sensors.iter().any(|sensor| {
            let distance = (sensor.x - x).abs() + (sensor.y - y).abs();
            distance <= sensor.range
        })
    }
}

impl From<&str> for Sensors {
    fn from(s: &str) -> Self {
        let sensors_and_beacons: Vec<Sensor> = s
            .trim()
            .lines()
            .map(|line| {
                let (sensor_raw, beacon_raw) =
                    line.split_once(": closest beacon is at x=").unwrap();
                let sensor: (i32, i32) = sensor_raw
                    .trim_start_matches("Sensor at x=")
                    .split(", y=")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                let beacon: (i32, i32) = beacon_raw
                    .split(", y=")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Sensor {
                    x: sensor.0,
                    y: sensor.1,
                    range: (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs(),
                }
            })
            .collect();
        Sensors {
            sensors: sensors_and_beacons,
        }
    }
}

impl From<(&Sensors, usize, i32, &mut Vec<Cell>)> for Row {
    fn from((sensors, max, current_row, cells): (&Sensors, usize, i32, &mut Vec<Cell>)) -> Self {
        // let mut cells = vec![Cell::Empty; max + 1];

        Row {
            solution: cells.iter().enumerate().find_map(|(x, _)| {
                if sensors.is_covered(x as i32, current_row) {
                    None
                } else {
                    Some(x)
                }
            }),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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

pub fn solution(input: &str, max: usize) -> usize {
    let mut cells = vec![Cell::Empty; max + 1];
    let sensors = Sensors::from(input);
    for i in 0..max {
        println!("row: {}", i);
        let grid = Row::from((&sensors, max, i as i32, &mut cells));
        cells.iter_mut().for_each(|cell| {
            *cell = Cell::Empty;
        });
        if let Some(solution) = grid.solution {
            return solution * 4000000 + i;
        }
    }
    panic!("No solution found");
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
                20
            ),
            56000011
        );
    }
}
