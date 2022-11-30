use std::collections::HashSet;
use std::fs::read_to_string;

pub fn calculate_adapters() -> usize {
    let file_content = read_to_string("day10/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let adapters: HashSet<usize> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let mut current_adapter: usize = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 1;
    loop {
        match current_adapter {
            found if adapters.contains(&(found + 1)) => {
                diff_1 += 1;
                current_adapter += 1;
            }
            found if adapters.contains(&(found + 2)) => current_adapter += 2,
            found if adapters.contains(&(found + 3)) => {
                diff_3 += 1;
                current_adapter += 3;
            }
            _ => return diff_1 * diff_3,
        }
    }
}
