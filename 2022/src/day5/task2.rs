use crate::day5::task1::parse_initial_state;
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
        stacks[to].append(&mut from_elements);
    });

    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .join("")
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
            "MCD"
        );
    }
}
