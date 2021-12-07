use crate::utils::load_input;
use itertools::Itertools;

fn median(array: &Vec<usize>) -> f64 {
    if (array.len() % 2) == 0 {
        let index_left = array.len() / 2 - 1;
        let index_right = array.len() / 2;
        (array[index_left] + array[index_right]) as f64 / 2.0
    } else {
        array[(array.len() / 2)] as f64
    }
}

pub fn solve() -> usize {
    let lines = load_input(7);
    let crabs: Vec<usize> = lines[0]
        .split(',')
        .into_iter()
        .map(|crab| crab.parse().unwrap())
        .sorted()
        .collect();
    let median_position = median(&crabs);
    crabs
        .into_iter()
        .map(|pos| (pos as f64 - median_position).abs() as usize)
        .sum()
}
