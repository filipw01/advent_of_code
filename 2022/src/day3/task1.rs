use itertools::Itertools;
use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let lines = input.lines().map_into::<String>();
    lines
        .map(|line| {
            let chars = line.chars();
            let first_compartment: HashSet<_> = chars.clone().take(line.len() / 2).collect();
            let second_compartment: HashSet<_> = chars.skip(line.len() / 2).collect();
            let duplicate = first_compartment
                .intersection(&second_compartment)
                .next()
                .unwrap();
            if duplicate.is_uppercase() {
                *duplicate as usize - 38
            } else {
                *duplicate as usize - 96
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
            ),
            157
        );
    }
}
