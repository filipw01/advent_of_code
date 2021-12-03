use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(2);
    let str_lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (x, z, _) = str_lines
        .into_iter()
        .map(|line| line.split_once(' ').unwrap())
        .fold((0, 0, 0), |(x, z, aim), (command, value)| {
            match (command, value.parse::<usize>().unwrap()) {
                ("forward", x_move) => (x + x_move, z + x_move * aim, aim),
                ("down", aim_move) => (x, z, aim + aim_move),
                ("up", aim_move) => (x, z, aim - aim_move),
                _ => unreachable!(),
            }
        });
    x * z
}
