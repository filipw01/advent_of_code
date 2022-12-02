use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map_into::<String>().collect();
    lines.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(""), 0);
    }
}
