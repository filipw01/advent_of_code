use crate::utils::load_input;
use itertools::Itertools;

pub fn solve() -> usize {
    let lines = load_input(2);
    let (x, z, _) = lines.into_iter().fold((0, 0, 0), |(x, z, aim), line| {
        match line.splitn(2, ' ').collect_tuple().unwrap() {
            ("forward", x_move) => (
                x + x_move.parse::<usize>().unwrap(),
                z + x_move.parse::<usize>().unwrap() * aim,
                aim,
            ),
            ("down", aim_move) => (x, z, aim + aim_move.parse::<usize>().unwrap()),
            ("up", aim_move) => (x, z, aim - aim_move.parse::<usize>().unwrap()),
            _ => unreachable!(),
        }
    });
    x * z
}
