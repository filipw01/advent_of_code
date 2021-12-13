use crate::utils::load_input;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() -> usize {
    let lines = load_input(6);
    let mut school: HashMap<usize, usize> = lines
        .get(0)
        .unwrap()
        .split(',')
        .into_iter()
        .map(|line| line.parse().unwrap())
        .counts_by(|line| line);

    for _ in 0..256 {
        let mut next_gen = HashMap::new();
        for (timer, fish_count) in school {
            if timer == 0 {
                next_gen.insert(8, fish_count);
                let old_fish_count = next_gen.get(&(6 as usize)).unwrap_or(&(0 as usize)).clone();
                next_gen.insert(6, fish_count + old_fish_count);
            } else {
                let old_fish_count = next_gen
                    .get(&(timer - 1 as usize))
                    .unwrap_or(&(0 as usize))
                    .clone();
                next_gen.insert(timer - 1, fish_count + old_fish_count);
            }
        }
        school = next_gen;
    }
    school.values().sum()
}
