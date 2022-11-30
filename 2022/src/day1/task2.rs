use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    input.len()
}

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("input"), 5);
    }
}
