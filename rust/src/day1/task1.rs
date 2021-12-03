use crate::utils::load_input;
use itertools::Itertools;

pub fn calculate_depth_increase() -> usize {
    let lines = load_input(1);
    lines
        .into_iter()
        .map(|line| line.parse::<u16>().expect("Received invalid u16"))
        .tuple_windows()
        .filter(|(prev, current)| current > prev)
        .count()
}
