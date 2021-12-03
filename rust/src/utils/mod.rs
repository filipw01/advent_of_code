use std::fs::read_to_string;
use itertools::Itertools;

pub fn load_input(day: u8) -> Vec<String> {
    let file_content = read_to_string(format!("src/day{}/data.txt", day))
        .expect("Failed to load data.txt file for day {}");
    let mut lines:Vec<String> = file_content.split('\n').map_into::<String>().collect();
    lines.pop();
    lines
}
