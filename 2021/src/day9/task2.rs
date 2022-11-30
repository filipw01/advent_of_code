use crate::utils::load_input;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() -> usize {
    let lines = load_input(9);
    let cells: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut basin_sizes = vec![];
    for y in 0..cells.len() {
        for x in 0..cells.get(y).unwrap().len() {
            let pre_x = x as isize - 1;
            let pre_y = y as isize - 1;
            let main_cell = cells.get(y).unwrap().get(x).unwrap();
            let empty_vec = vec![];
            let side_cells = [
                cells
                    .get(pre_y as usize)
                    .unwrap_or(&empty_vec)
                    .get(x)
                    .unwrap_or(&10),
                cells.get(y + 1).unwrap_or(&empty_vec).get(x).unwrap_or(&10),
                cells.get(y).unwrap().get(pre_x as usize).unwrap_or(&10),
                cells.get(y).unwrap().get(x + 1).unwrap_or(&10),
            ];
            if side_cells.into_iter().all(|cell| cell > main_cell) {
                let mut basin = HashSet::new();
                count_basin_cells(&cells, (x, y), &mut basin);
                basin_sizes.push(basin.len());
            }
        }
    }
    let mut basin_iter = basin_sizes.into_iter().sorted().rev();
    basin_iter.next().unwrap() * basin_iter.next().unwrap() * basin_iter.next().unwrap()
}

fn count_basin_cells(
    cells: &Vec<Vec<u32>>,
    (x, y): (usize, usize),
    basin: &mut HashSet<(usize, usize)>,
) {
    basin.insert((x, y));
    let pre_x = (x as isize - 1) as usize;
    let pre_y = (y as isize - 1) as usize;
    let main_cell = cells.get(y).unwrap().get(x).unwrap();
    let empty_vec = vec![];
    let side_cells = [(x, pre_y), (x, y + 1), (pre_x, y), (x + 1, y)];
    side_cells.into_iter().for_each(|(x, y)| {
        let cell_set = HashSet::from([(x, y)]);
        if !basin.is_superset(&cell_set)
            && cells.get(y).unwrap_or(&empty_vec).get(x).unwrap_or(&9) < &9
            && main_cell < cells.get(y).unwrap().get(x).unwrap()
        {
            count_basin_cells(cells, (x, y), basin);
        }
    })
}
