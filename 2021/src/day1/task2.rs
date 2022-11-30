use crate::utils::load_input;
use itertools::Itertools;

pub fn calculate_depth_increase() -> usize {
    let lines = load_input(1);
    lines
        .into_iter()
        .map(|x| x.parse::<u16>().expect("Couldn't parse to u16"))
        .tuple_windows()
        .filter(|(a, _, _, d)| a < d)
        .count()
}
