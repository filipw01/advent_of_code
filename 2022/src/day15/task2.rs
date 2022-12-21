use itertools::Itertools;

struct Grid {
    solution: (i32, i32),
}

impl From<(&str, i32)> for Grid {
    fn from((s, max): (&str, i32)) -> Self {
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
        let solution = sensors_and_beacons
            .iter()
            .find_map(|((x, y), (bx, by))| {
                println!("sensor: {},{}", x, y);
                let distance = (x - bx).abs() + (y - by).abs();
                let distance_plus_one = distance + 1;
                (0..=distance_plus_one).find_map(|i| {
                    let top_left = (x - i, y - distance_plus_one + i);
                    let top_right = (x + i, y - distance_plus_one + i);
                    let bottom_left = (x - i, y + distance_plus_one - i);
                    let bottom_right = (x + i, y + distance_plus_one - i);
                    vec![top_left, top_right, bottom_left, bottom_right]
                        .into_iter()
                        .find(|(x, y)| {
                            if !(*x >= 0 && *x <= max && *y >= 0 && *y <= max) {
                                return false;
                            }
                            sensors_and_beacons.iter().all(|((sx, sy), (bx, by))| {
                                let distance = (x - sx).abs() + (y - sy).abs();
                                let range = (sx - bx).abs() + (sy - by).abs();
                                distance > range
                            })
                        })
                })
            })
            .unwrap();

        Grid { solution }
    }
}

pub fn solution(input: &str, max: i32) -> u64 {
    let grid = Grid::from((input, max));
    ((grid.solution.0 as u64 * 4000000) + grid.solution.1 as u64) as u64
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
