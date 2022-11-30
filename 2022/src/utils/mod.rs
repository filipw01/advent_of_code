use std::fs::read_to_string;

pub fn load_input(day: u8) -> String {
    read_to_string(format!("src/day{}/data.txt", day))
        .expect("Failed to load data.txt file for day {}")
}
