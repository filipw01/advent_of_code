use std::collections::HashMap;
use std::fs::read_to_string;

pub fn emulate() -> usize {
    let file_content = read_to_string("day14/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut mask = "";
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" = ").collect();
        let (operation, value) = (split[0], split[1]);
        if operation == "mask" {
            mask = value;
        }
        if operation.starts_with("mem") {
            let address = operation
                .get(4..operation.len() - 1)
                .unwrap()
                .parse()
                .unwrap();
            let value = apply_mask(value, mask);
            memory.insert(address, value);
        }
    }
    memory.values().sum()
}

fn apply_mask(value: &str, mask: &str) -> usize {
    let value = format!("{:b}", value.parse::<usize>().unwrap())
        .chars()
        .rev()
        .collect::<String>();
    let mask = mask.chars().rev().collect::<String>();
    let mut output_number = "".to_string();
    for index in 0..mask.len() {
        if mask.chars().nth(index).unwrap() == 'X' {
            if index < value.len() {
                output_number.push(value.chars().nth(index).unwrap())
            } else {
                output_number.push('0')
            }
        } else {
            output_number.push(mask.chars().nth(index).unwrap());
        }
    }
    output_number = output_number.chars().rev().collect::<String>();
    isize::from_str_radix(output_number.as_str(), 2).unwrap() as usize
}
