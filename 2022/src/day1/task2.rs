use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines: Vec<String> = input.lines().map_into::<String>().collect();

    lines
        .into_iter()
        .group_by(|calories| !calories.is_empty())
        .into_iter()
        .filter(|(is_elf, _)| *is_elf)
        .map(|(_, calories_group)| {
            calories_group
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
            ),
            45000
        );
    }
}
