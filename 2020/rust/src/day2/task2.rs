use std::fs::read_to_string;

pub fn check_password() -> Result<usize, &'static str> {
    let file_content = read_to_string("day2/data.txt").unwrap();
    let mut lines: Vec<&str> = file_content.split("\n").collect();
    lines.pop();
    struct Policy {
        password: String,
        letter: char,
        first_index: usize,
        last_index: usize,
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
            let first_index = split_3.get(0).unwrap() - 1;
            let last_index = split_3.get(1).unwrap() - 1;
            Policy {
                password: password.to_string(),
                letter: letter.chars().next().unwrap(),
                first_index: first_index as usize,
                last_index: last_index as usize,
            }
        })
        .filter(|policy| {
            let letter_count = policy
                .password
                .chars()
                .enumerate()
                .filter(|(index, _)| *index == policy.first_index || *index == policy.last_index)
                .filter(|(_, char)| *char == policy.letter)
                .count();
            letter_count == 1
        })
        .map(|policy| policy.password)
        .collect();
    return Ok(valid_passwords.len());
}
