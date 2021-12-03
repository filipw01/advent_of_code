use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(2);
    let str_lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (x, z) = str_lines
        .into_iter()
        .map(|line| line.split_once(' ').unwrap())
        .fold((0, 0), |(x, z), (command, value)| {
            match (command, value.parse::<usize>().unwrap()) {
                ("forward", x_move) => (x + x_move, z),
                ("down", z_move) => (x, z + z_move),
                ("up", z_move) => (x, z - z_move),
                _ => unreachable!(),
            }
        });
    x * z
}
