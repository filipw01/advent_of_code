use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(8);
    let known_digits: usize = lines
        .into_iter()
        .map(|line| {
            let (_, output) = line.split_once(" | ").unwrap();
            output
                .split(' ')
                .filter(|wires| [2, 3, 4, 7].contains(&(wires.len() as i32)))
                .count()
        })
        .sum();
    known_digits
}
