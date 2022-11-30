use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("input"), 5);
    }
}
