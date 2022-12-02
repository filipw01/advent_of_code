use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines = input.lines().map_into::<String>();
    lines
        .map(|line| {
            let split: Vec<_> = line.split(' ').take(2).collect();
            let opponent = split[0];
            let your_move = split[1];
            match_score(your_move, opponent)
        })
        .sum()
}

pub fn match_score(your_move: &str, opponent_move: &str) -> usize {
    match (your_move, opponent_move) {
        ("X", "A") => 3 + 1,
        ("X", "B") => 0 + 1,
        ("X", "C") => 6 + 1,
        ("Y", "A") => 6 + 2,
        ("Y", "B") => 3 + 2,
        ("Y", "C") => 0 + 2,
        ("Z", "A") => 0 + 3,
        ("Z", "B") => 6 + 3,
        ("Z", "C") => 3 + 3,
        _ => panic!("Unknown move"),
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
            15
        );
    }
}
