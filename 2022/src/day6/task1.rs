use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    input
        .chars()
        .tuple_windows()
        .find_position(|(a, b, c, d)| [a, b, c, d].into_iter().all_unique())
        .unwrap()
        .0
        + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_solution_3() {
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test_solution_4() {
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn test_solution_5() {
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
