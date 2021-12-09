use crate::utils::load_input;

pub fn solve() -> u32 {
    let lines = load_input(9);
    let cells: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut low_points = vec![];
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
                low_points.push(main_cell + 1);
            }
        }
    }
    low_points.into_iter().sum()
}
