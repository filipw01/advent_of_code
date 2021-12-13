use itertools::Itertools;
use std::fs::read_to_string;

pub fn load_input(day: u8) -> Vec<String> {
    let file_content = read_to_string(format!("src/day{}/data.txt", day))
        .expect("Failed to load data.txt file for day {}");
    let lines: Vec<String> = file_content.lines().map_into::<String>().collect();
    lines
}
