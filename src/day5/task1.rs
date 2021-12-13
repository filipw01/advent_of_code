use crate::utils::load_input;

#[derive(Debug)]
struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub fn solve() -> usize {
    let lines = load_input(5);
    let mut ocean_floor = [[0; 1000]; 1000];
    lines
        .into_iter()
        .map(|line| {
            let (start, end) = line.split_once(" -> ").unwrap();
            let (x1, y1) = start.split_once(",").unwrap();
            let (x2, y2) = end.split_once(",").unwrap();
            Vent {
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
            }
        })
        .filter(|vent| vent.x1 == vent.x2 || vent.y1 == vent.y2)
        .for_each(|vent| {
            if vent.x1 == vent.x2 {
                let range = if vent.y1 < vent.y2 {
                    vent.y1..vent.y2 + 1
                } else {
                    vent.y2..vent.y1 + 1
                };
                for y in range {
                    ocean_floor[y][vent.x1] += 1
                }
            } else {
                let range = if vent.x1 < vent.x2 {
                    vent.x1..vent.x2 + 1
                } else {
                    vent.x2..vent.x1 + 1
                };
                for x in range {
                    ocean_floor[vent.y1][x] += 1
                }
            };
        });

    let crossings_count = ocean_floor
        .iter()
        .map(|row| row.iter().filter(|cell| **cell > 1).count())
        .sum();

    crossings_count
}
