use std::fs::read_to_string;

pub fn calculate_encounters(right_move: usize, down_move: usize) -> Result<usize, &'static str> {
    let file_content = read_to_string("day3/data.txt").unwrap();
    let mut lines: Vec<&str> = file_content.split("\n").collect();
    lines.pop();
    let mut x = 0;
    let mut trees = 0;
    for (_, line) in lines
        .iter()
        .enumerate()
        .filter(|(index, _)| index % down_move == 0)
    {
        let index = x % line.len();
        let cell = line.chars().nth(index).unwrap();
        if cell == '#' {
            trees += 1;
        }
        x += right_move;
    }
    return Ok(trees);
}

pub fn calculate_solution() -> Result<usize, &'static str> {
    let solution = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (right, down)| {
            acc * calculate_encounters(*right, *down).unwrap()
        });
    return Ok(solution);
}
