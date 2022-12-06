use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    find_first_package(input, 4)
}

pub fn find_first_package(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .position(|window| window.iter().all_unique())
        .unwrap()
        + window_size
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
