use std::fs::read_to_string;

pub fn check_password() -> Result<usize, &'static str> {
    let file_content = read_to_string("day2/data.txt").unwrap();
    let mut lines: Vec<&str> = file_content.split("\n").collect();
    lines.pop();
    struct Policy {
        password: String,
        letter: char,
        min_count: usize,
        max_count: usize,
    }
    let valid_passwords: Vec<_> = lines
        .iter()
        .map(|line| {
            let split_1: Vec<_> = line.split(": ").collect();
            let password = split_1.get(1).unwrap();
            let split_2: Vec<_> = split_1.get(0).unwrap().split(" ").collect();
            let letter = split_2.get(1).unwrap();
            let split_3: Vec<i32> = split_2
                .get(0)
                .unwrap()
                .split("-")
                .map(|count| count.parse().unwrap())
                .collect();
            let min_count = split_3.get(0).unwrap();
            let max_count = split_3.get(1).unwrap();
            Policy {
                password: password.to_string(),
                letter: letter.chars().next().unwrap(),
                min_count: *min_count as usize,
                max_count: *max_count as usize,
            }
        })
        .filter(|policy| {
            let letter_count = policy
                .password
                .chars()
                .filter(|char| *char == policy.letter)
                .count();
            policy.min_count <= letter_count && letter_count <= policy.max_count
        })
        .map(|policy| policy.password)
        .collect();
    return Ok(valid_passwords.len());
}
