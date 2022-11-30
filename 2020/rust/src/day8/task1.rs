use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() -> i32 {
    let file_content = read_to_string("day8/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut accumulator = 0;
    let mut executed_instructions: HashSet<i32> = HashSet::new();
    let mut i: i32 = 0;
    loop {
        let line = lines.get(i as usize).unwrap();
        if executed_instructions.contains(&i) {
            return accumulator;
        } else {
            executed_instructions.insert(i);
        }
        let split: Vec<_> = line.split(' ').collect();
        let instruction = split.get(0).unwrap();
        let number: i32 = split.get(1).unwrap().parse().unwrap();
        match *instruction {
            "acc" => {
                accumulator += number;
                i += 1
            }
            "jmp" => i += number,
            "nop" => i += 1,
            _ => unreachable!(),
        }
    }
}
