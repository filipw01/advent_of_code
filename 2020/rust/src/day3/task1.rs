use std::fs::read_to_string;

pub fn calculate_encounters() -> Result<usize, &'static str> {
    let file_content = read_to_string("day3/data.txt").unwrap();
    let mut lines: Vec<&str> = file_content.split("\n").collect();
    lines.pop();
    let mut x = 0;
    let mut trees = 0;
    for line in lines {
        let index = x % line.len();
        let cell = line.chars().nth(index).unwrap();
        if cell == '#' {
            trees += 1;
        }
        x += 3;
    }
    return Ok(trees);
}
