use std::fs::read_to_string;

pub fn calculate_expense_report() -> Result<i32, &'static str> {
    let file_content = read_to_string("day1/data.txt").unwrap();
    let mut lines: Vec<&str> = file_content.split("\n").collect();
    lines.pop();
    for (index_1, line_1) in lines.iter().enumerate() {
        let number_1: i32 = line_1.parse().unwrap();
        for (index_2, line_2) in lines.iter().enumerate() {
            let number_2: i32 = line_2.parse().unwrap();
            if index_1 == index_2 {
                continue;
            }
            if number_1 + number_2 == 2020 {
                return Ok(number_1 * number_2);
            }
        }
    }
    Err("No solution")
}
