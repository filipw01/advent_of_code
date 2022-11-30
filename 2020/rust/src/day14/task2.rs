use std::collections::HashMap;
use std::fs::read_to_string;

pub fn emulate() -> usize {
    let file_content = read_to_string("day14/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut mask = "";
    let mut memory: HashMap<String, usize> = HashMap::new();
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
            let possible_addresses = calculate_addresses(address, mask);
            for masked_address in possible_addresses {
                memory.insert(masked_address, value.parse().unwrap());
            }
        }
    }
    memory.values().sum()
}

fn calculate_addresses<'a>(address: usize, mask: &'a str) -> Vec<String> {
    let value = format!("{:b}", address).chars().rev().collect::<String>();
    let mask = mask.chars().rev().collect::<String>();
    let mut address = "".to_string();
    for index in 0..mask.len() {
        if mask.chars().nth(index).unwrap() == '0' {
            if index < value.len() {
                address.push(value.chars().nth(index).unwrap())
            } else {
                address.push('0')
            }
        } else {
            address.push(mask.chars().nth(index).unwrap());
        }
    }
    address = address.chars().rev().collect::<String>();
    let mut possible_address: Vec<String> = Vec::new();
    let x_count = address.chars().filter(|c| *c == 'X').count();
    let mut values_to_insert = "1".repeat(x_count);
    loop {
        let mut temp_address = address.clone();
        for bit in values_to_insert.chars() {
            temp_address = temp_address.replacen('X', bit.to_string().as_str(), 1);
        }
        possible_address.push(temp_address.clone());
        if isize::from_str_radix(values_to_insert.as_str(), 2).unwrap() == 0 {
            return possible_address;
        }
        let temp_mask_replacement =
            isize::from_str_radix(values_to_insert.as_str(), 2).unwrap() as usize;
        values_to_insert = format!(
            "{:0width$}",
            format!("{:b}", (temp_mask_replacement - 1))
                .parse::<usize>()
                .unwrap(),
            width = x_count
        );
    }
}
