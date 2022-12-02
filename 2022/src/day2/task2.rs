use crate::day2::task1::match_score;
use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines = input.lines().map_into::<String>();
    lines
        .map(|line| {
            let split: Vec<_> = line.split(' ').take(2).collect();
            let opponent = split[0];
            let strategy = split[1];
            let your_move = find_your_move(strategy, opponent);
            match_score(your_move, opponent)
        })
        .sum()
}

fn find_your_move<'a>(strategy: &'a str, opponent: &'a str) -> &'a str {
    match (strategy, opponent) {
        ("X", "A") => "Z",
        ("X", "B") => "X",
        ("X", "C") => "Y",
        ("Y", "A") => "X",
        ("Y", "B") => "Y",
        ("Y", "C") => "Z",
        ("Z", "A") => "Y",
        ("Z", "B") => "Z",
        ("Z", "C") => "X",
        _ => panic!("Can't find your move"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "A Y
B X
C Z
"
            ),
            12
        );
    }
}
