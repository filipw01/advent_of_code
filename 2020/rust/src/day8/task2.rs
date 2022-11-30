use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() -> i32 {
    let file_content = read_to_string("day8/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut original_nth_changed = 0;
    let mut nth_changed = 0;
    loop {
        let mut broken = false;
        let mut i: i32 = 0;
        let mut accumulator = 0;
        let mut executed_instructions: HashSet<i32> = HashSet::new();
        while i < lines.len() as i32 {
            let line = lines.get(i as usize).unwrap();
            if executed_instructions.contains(&i) {
                broken = true;
                break;
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
                "jmp" => {
                    if nth_changed == 0 {
                        i += 1;
                    } else {
                        i += number
                    }
                    nth_changed -= 1;
                }
                "nop" => {
                    if nth_changed == 0 {
                        i += number
                    } else {
                        i += 1;
                    }
                    nth_changed -= 1;
                }
                _ => unreachable!(),
            }
        }
        if broken == false {
            return accumulator;
        }
        original_nth_changed += 1;
        nth_changed = original_nth_changed;
    }
}
