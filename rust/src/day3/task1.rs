use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(3);
    let num_lines = lines.len();
    let width = lines[0].len();
    let gamma = lines
        .into_iter()
        .map(|line| usize::from_str_radix(&*line, 2).unwrap())
        .fold(vec![0 as usize; width], |acc, line| {
            acc.into_iter()
                .enumerate()
                // increment acc if bit on the correct position equals 1
                .map(|(index, acc_item)| acc_item + ((line & (1 << index)) >> index))
                .collect()
        })
        .into_iter()
        .enumerate()
        // set 1 or 0 on correct position and 0 on all other to sum later into final gamma
        .map(|(index, gamma_bit)| ((gamma_bit >= num_lines / 2) as usize) << index)
        .sum::<usize>();

    // logical not capped at {width} bits
    let epsilon = !gamma & ((1 << width) - 1);
    gamma * epsilon
}
