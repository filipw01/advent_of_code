use crate::day9::task1::{calculate_new_position, parse_input, Direction};
use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let mut visited_positions = HashSet::new();
    let mut rope = [(0, 0); 10];
    parse_input(input)
        .into_iter()
        .for_each(|(direction, value)| {
            let move_by = match direction {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            for _ in 0..value {
                visit(&mut rope, &mut visited_positions, move_by);
            }
        });
    visited_positions.len()
}

fn visit(rope: &mut [(i32, i32); 10], visited: &mut HashSet<(i32, i32)>, move_by: (i32, i32)) {
    rope[0].0 += move_by.0;
    rope[0].1 += move_by.1;

    for i in 1..rope.len() {
        if rope[i - 1].0.abs_diff(rope[i].0) > 1 || rope[i - 1].1.abs_diff(rope[i].1) > 1 {
            rope[i].0 = calculate_new_position(rope[i - 1].0, rope[i].0);
            rope[i].1 = calculate_new_position(rope[i - 1].1, rope[i].1);
        }
    }

    visited.insert(*rope.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
            ),
            1
        );
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(
            solution(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"
            ),
            36
        );
    }
}
