use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines: Vec<String> = input.lines().map_into::<String>().collect();
    lines.len()
}

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("input"), 1);
    }
}
