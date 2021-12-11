use itertools::Itertools;
use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(10);
    let scores: Vec<usize> = lines
        .iter()
        .filter_map(|line| {
            let mut stack = vec![];
            let score = line
                .chars()
                .find_map(|c| match c {
                    '<' | '{' | '[' | '(' => {
                        stack.push(c);
                        None
                    }
                    '>' => {
                        if check_closing_char(c, *stack.last().unwrap()) {
                            stack.pop();
                            None
                        } else {
                            Some(25137)
                        }
                    }
                    '}' => {
                        if check_closing_char(c, *stack.last().unwrap()) {
                            stack.pop();
                            None
                        } else {
                            Some(1197)
                        }
                    }
                    ']' => {
                        if check_closing_char(c, *stack.last().unwrap()) {
                            stack.pop();
                            None
                        } else {
                            Some(57)
                        }
                    }
                    ')' => {
                        if check_closing_char(c, *stack.last().unwrap()) {
                            stack.pop();
                            None
                        } else {
                            Some(3)
                        }
                    }
                    unexpected => panic!("Got unexpected character: {}", unexpected),
                })
                .unwrap_or(0);
            if score == 0 {
                Some(stack.into_iter().rev().fold(0, |score, char| {
                    score * 5
                        + match char {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            unexpected => panic!("Got unexpected character: {}", unexpected),
                        }
                }))
            } else {
                None
            }
        })
        .sorted()
        .collect();
    *scores.get(scores.len() / 2).unwrap()
}

fn check_closing_char(received: char, expected_match: char) -> bool {
    (received == '>' && expected_match == '<')
        || (received == '}' && expected_match == '{')
        || (received == ']' && expected_match == '[')
        || (received == ')' && expected_match == '(')
}
