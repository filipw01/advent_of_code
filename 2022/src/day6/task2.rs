use itertools::Itertools;
use std::collections::VecDeque;

pub fn solution(input: &str) -> usize {
    let mut window = VecDeque::from(input.chars().take(14).collect_vec());
    for (index, char) in input.chars().skip(14).enumerate() {
        if !window.iter().all_unique() {
            window.pop_front();
            window.push_back(char);
        } else {
            return index + 14;
        }
    }
    panic!("No solution found");
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
