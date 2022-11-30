use std::fs::read_to_string;
use std::collections::HashMap;

pub fn play() -> usize {
    let file_content = read_to_string("day15/data.txt").unwrap();
    let numbers: Vec<usize> = file_content.split("\n").nth(0).unwrap()
        .split(',').map(|num| num.parse().unwrap()).collect();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut last_number = *numbers.last().unwrap();
    let mut turn = 1;
    for index in 0..numbers.len() - 1 {
        memory.insert(numbers[index], turn);
        turn += 1;
    }
    loop {
        if memory.contains_key(&last_number) {
            let last_occurrence = *memory.get(&last_number).unwrap();
            memory.insert(last_number, turn);
            last_number = turn - last_occurrence;
        } else {
            memory.insert(last_number, turn);
            last_number = 0;
        }
        turn += 1;
        if turn == 30000000 {
            return last_number;
        }
    }
}
