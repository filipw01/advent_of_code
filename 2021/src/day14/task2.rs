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
    let mut end_polymer: HashMap<String, usize> = initial
        .chars()
        .tuple_windows()
        .map(|(c1, c2)| {
            let pair = format!("{}{}", c1, c2);
            (
                pair.clone(),
                initial
                    .chars()
                    .tuple_windows()
                    .filter(|(inner_c1, inner_c2)| format!("{}{}", inner_c1, inner_c2) == pair)
                    .count(),
            )
        })
        .collect();
    for _ in 0..40 {
        insert_matching_letters(&mut end_polymer, &instructions);
    }
    let mut letters_count: HashMap<char, usize> = HashMap::from([
        (initial.chars().next().unwrap(), 1),
        (initial.chars().last().unwrap(), 1),
    ]);
    end_polymer.iter().for_each(|(pair, count)| {
        let c1 = pair.chars().next().unwrap();
        if let Some(old_count) = letters_count.get(&c1) {
            let old_count = old_count.clone();
            letters_count.insert(c1, old_count + count);
        } else {
            letters_count.insert(c1, *count);
        }
        let c2 = pair.chars().last().unwrap();
        if let Some(old_count) = letters_count.get(&c2) {
            let old_count = old_count.clone();
            letters_count.insert(c2, old_count + count);
        } else {
            letters_count.insert(c2, *count);
        }
    });
    let max = letters_count.values().max().unwrap();
    let min = letters_count.values().min().unwrap();
    (max - min) / 2
}

fn insert_matching_letters(
    polymer: &mut HashMap<String, usize>,
    instructions: &HashMap<&str, &str>,
) {
    let mut new_polymer: HashMap<String, usize> = HashMap::new();
    polymer.iter().for_each(|(pair, count)| {
        let matching_letter = instructions.get(pair.as_str()).unwrap();
        let new_pair1 = format!("{}{}", pair.chars().next().unwrap(), matching_letter);
        if let Some(old_count) = new_polymer.get(new_pair1.as_str()) {
            let old_count = old_count.clone();
            new_polymer.insert(new_pair1, old_count + *count);
        } else {
            new_polymer.insert(new_pair1, *count);
        }
        let new_pair2 = format!("{}{}", matching_letter, pair.chars().last().unwrap());
        if let Some(old_count) = new_polymer.get(new_pair2.as_str()) {
            let old_count = old_count.clone();
            new_polymer.insert(new_pair2, old_count + *count);
        } else {
            new_polymer.insert(new_pair2, *count);
        }
    });
    *polymer = new_polymer
}
