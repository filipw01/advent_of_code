use crate::utils::load_input;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() -> usize {
    let input = load_input(14).join("\n");
    let (initial, instructions) = input.split_once("\n\n").unwrap();
    let instructions: HashMap<&str, &str> = instructions
        .split('\n')
        .map(|instruction| instruction.split_once(" -> ").unwrap())
        .collect();
    let mut end_polymer = initial.to_string();
    for _ in 0..10 {
        insert_matching_letters(&mut end_polymer, &instructions);
    }
    let mut letters_count: HashMap<char, usize> = HashMap::new();
    end_polymer.chars().for_each(|c| {
        if let Some(count) = letters_count.get(&c) {
            letters_count.insert(c, count + 1);
        } else {
            letters_count.insert(c, 1);
        }
    });
    let max = letters_count.values().max().unwrap();
    let min = letters_count.values().min().unwrap();
    max - min
}

fn insert_matching_letters(polymer: &mut String, instructions: &HashMap<&str, &str>) {
    let mut to_insert: Vec<&str> = vec![];
    polymer.chars().tuple_windows().for_each(|(c1, c2)| {
        let pair = format!("{}{}", c1, c2);
        to_insert.push(instructions.get(pair.as_str()).unwrap());
    });
    let mut result: Vec<String> = polymer
        .chars()
        .zip(to_insert.into_iter())
        .map(|(c, s)| format!("{}{}", c, s))
        .collect();
    result.push(polymer.chars().last().unwrap().to_string());
    *polymer = result.join("");
}
