use itertools::Itertools;
use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let lines = input.lines().map_into::<String>();
    lines
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let items = lines.map(|line| HashSet::<char>::from_iter(line.chars()));
            let badge = items
                .fold(HashSet::new(), |acc, item| {
                    if acc.is_empty() {
                        item
                    } else {
                        acc.intersection(&item).copied().collect()
                    }
                })
                .drain()
                .next()
                .unwrap();
            if badge.is_uppercase() {
                badge as usize - 38
            } else {
                badge as usize - 96
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
            70
        );
    }
}
