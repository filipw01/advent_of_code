use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let mut visited_positions = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    parse_input(input)
        .into_iter()
        .for_each(|(direction, value)| {
            let move_by = match direction {
                Direction::Up => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Down => (0, -1),
                Direction::Right => (1, 0),
            };
            (0..value).for_each(|_| visit(&mut head, &mut tail, &mut visited_positions, move_by))
        });
    visited_positions.len()
}

fn visit(
    head: &mut (i32, i32),
    tail: &mut (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    move_by: (i32, i32),
) {
    head.0 += move_by.0;
    head.1 += move_by.1;

    if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
        tail.0 = calculate_new_position(head.0, tail.0);
        tail.1 = calculate_new_position(head.1, tail.1);
    }
    visited.insert(*tail);
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_input(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let direction = match split.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                value => panic!("Unable to parse the direction, got {}", value),
            };
            let value = split.next().unwrap().parse::<usize>().unwrap();
            (direction, value)
        })
        .collect()
}

pub fn calculate_new_position(head: i32, tail: i32) -> i32 {
    let diff = head.abs_diff(tail);
    if diff == 1 {
        head
    } else {
        (head + tail) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_new_position() {
        assert_eq!(calculate_new_position(1, 3), 2);
        assert_eq!(calculate_new_position(1, 2), 1);
        assert_eq!(calculate_new_position(2, 1), 2);
        assert_eq!(calculate_new_position(3, 1), 2);
        assert_eq!(calculate_new_position(-2, -1), -2);
        assert_eq!(calculate_new_position(-1, 0), -1);
        assert_eq!(calculate_new_position(0, -1), 0);
    }
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
            13
        );
    }
}
