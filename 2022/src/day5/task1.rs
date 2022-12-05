use itertools::Itertools;

pub fn solution(input: &str) -> String {
    let (initial, moves): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = parse_initial_state(initial);

    moves.lines().for_each(|manipulation| {
        let mut split = manipulation.split(' ');
        let _ = split.next();
        let count: usize = split.next().unwrap().parse().unwrap();
        let _ = split.next();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let _ = split.next();
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let stack_len = stacks[from].len();
        let mut from_elements = stacks[from].drain(stack_len - count..).collect::<Vec<_>>();
        from_elements.reverse();
        stacks[to].append(&mut from_elements);
    });

    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .join("")
}

pub fn parse_initial_state(initial: &str) -> Vec<Vec<char>> {
    let lines = initial.lines().collect_vec();
    let mut stacks = vec![];
    for col_index in 0..lines.last().unwrap().len() {
        if col_index % 4 == 1 {
            let mut stack = vec![];
            for row_index in (0..lines.len() - 1).rev() {
                if let Some(char) = lines[row_index].chars().nth(col_index) {
                    if char != ' ' {
                        stack.push(char);
                    }
                }
            }
            stacks.push(stack);
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
            ),
            "CMZ"
        );
    }
}
