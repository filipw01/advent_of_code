use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(10);
    lines
        .iter()
        .map(|line| {
            let mut stack = vec![];
            line.chars()
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
                }).unwrap_or(0)
        })
        .sum()
}

fn check_closing_char(received: char, expected_match: char) -> bool {
    (received == '>' && expected_match == '<')
        || (received == '}' && expected_match == '{')
        || (received == ']' && expected_match == '[')
        || (received == ')' && expected_match == '(')
}
