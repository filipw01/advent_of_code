use crate::utils::load_input;
use std::collections::HashMap;

type Coord = (usize, usize);

pub fn solve() -> usize {
    let input = load_input(15);
    let size = input.len() - 1;
    let cave: HashMap<Coord, usize> = input
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x, y), c.to_digit(10).unwrap() as usize))
                .collect::<Vec<((usize, usize), usize)>>()
        })
        .collect();
    let mut paths_minimums: HashMap<Coord, usize> = HashMap::from([((0, 0), 0)]);
    let mut active_paths = paths_minimums.clone();
    while active_paths.len() != 0 {
        active_paths = walk_paths(&active_paths, &mut paths_minimums, &cave);
    }
    *paths_minimums.get(&(size, size)).unwrap()
}

fn walk_paths(
    active_paths: &HashMap<Coord, usize>,
    paths: &mut HashMap<Coord, usize>,
    cave: &HashMap<Coord, usize>,
) -> HashMap<Coord, usize> {
    let mut new_active_paths = HashMap::new();
    active_paths.into_iter().for_each(|((x, y), total_danger)| {
        let mut neighbours = vec![];
        if *x as isize - 1 > 0 {
            let x = x - 1;
            neighbours.push(((x, *y), cave.get(&(x, *y)).unwrap()))
        }
        if *y as isize - 1 > 0 {
            let y = y - 1;
            neighbours.push(((*x, y), cave.get(&(*x, y)).unwrap()))
        }
        if let Some(danger) = cave.get(&(*x + 1, *y)) {
            neighbours.push(((*x + 1, *y), danger))
        }
        if let Some(danger) = cave.get(&(*x, *y + 1)) {
            neighbours.push(((*x, *y + 1), danger))
        }
        neighbours.into_iter().for_each(|((x, y), danger)| {
            if let Some(old_total_danger) = paths.get(&(x, y)) {
                if total_danger + danger < *old_total_danger {
                    new_active_paths.insert((x, y), total_danger + danger);
                    paths.insert((x, y), total_danger + danger);
                }
            } else {
                new_active_paths.insert((x, y), total_danger + danger);
                paths.insert((x, y), total_danger + danger);
            }
        });
    });
    new_active_paths
}
