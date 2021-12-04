use crate::utils::load_input;

fn get_gamma(lines: &Vec<usize>, width: usize) -> usize {
    let num_lines = lines.len();
    lines
        .into_iter()
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
        .sum::<usize>()
}

fn get_epsilon(gamma: usize, width: usize) -> usize {
    // logical not capped at {width} bits
    !gamma & ((1 << width) - 1)
}

pub fn solve() -> usize {
    let lines = load_input(3);
    let width = lines[0].len();
    let lines: Vec<usize> = lines
        .into_iter()
        .map(|line| usize::from_str_radix(&*line, 2).unwrap())
        .collect();

    let mut co2 = lines.clone();
    let mut oxygen = lines.clone();
    let mut oxygen_result = 0;
    let mut co2_result = 0;

    for i in (0..width).into_iter().rev() {
        let gamma = get_gamma(&oxygen, width);
        if oxygen.len() == 1 {
            oxygen_result = oxygen[0];
            break;
        }
        oxygen = oxygen
            .into_iter()
            .filter(|line| (((gamma & (1 << i)) >> i) == ((line & (1 << i)) >> i)))
            .collect();
    }

    for i in (0..width).into_iter().rev() {
        let gamma = get_gamma(&co2, width);
        let epsilon = get_epsilon(gamma, width);
        if co2.len() == 1 {
            co2_result = co2[0];
            break;
        }
        co2 = co2
            .into_iter()
            .filter(|line| (((epsilon & (1 << i)) >> i) == ((line & (1 << i)) >> i)))
            .collect();
    }
    // gave up, no idea why it's not working, the python variant works though
    co2_result * oxygen_result
}
