use std::fs::read_to_string;

pub fn calculate_depth_increase() -> Result<i32, &'static str> {
    let file_content = read_to_string("day1/data.txt").unwrap();
    let mut lines: Vec<&str> = file_content.split("\n").collect();
    lines.pop();

    // solution

    Err("No solution")
}
