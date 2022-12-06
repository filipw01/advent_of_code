use crate::day6::task1::find_first_package;

pub fn solution(input: &str) -> usize {
    find_first_package(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn test_solution_3() {
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn test_solution_4() {
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn test_solution_5() {
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
