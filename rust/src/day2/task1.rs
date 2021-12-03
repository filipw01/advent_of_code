use crate::utils::load_input;
use itertools::Itertools;

pub fn solve() -> usize {
    let lines = load_input(2);
    let (x, z) = lines.into_iter().fold((0, 0), |(x, z), line| {
        match line.splitn(2, ' ').collect_tuple().unwrap() {
            ("forward", x_move) => (x + x_move.parse::<usize>().unwrap(), z),
            ("down", z_move) => (x, z + z_move.parse::<usize>().unwrap()),
            ("up", z_move) => (x, z - z_move.parse::<usize>().unwrap()),
            _ => unreachable!(),
        }
    });
    x * z
}
