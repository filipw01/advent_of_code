use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines = input.lines().map_into::<String>();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(""), 0);
    }
}
