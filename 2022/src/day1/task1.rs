use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines: Vec<String> = input.lines().map_into::<String>().collect();

    let mut max_calories = 0;
    let mut current_elf_calories = 0;

    for calories in lines {
        if calories.is_empty() {
            current_elf_calories = 0
        } else {
            current_elf_calories += calories.parse::<usize>().unwrap();
        }
        if current_elf_calories > max_calories {
            max_calories = current_elf_calories;
        }
    }

    max_calories
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
            24000
        );
    }
}
