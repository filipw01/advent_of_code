use crate::utils::load_input;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

fn get_n_bar_digits(n: usize, digits: &Vec<&str>) -> Vec<BTreeSet<char>> {
    digits
        .clone()
        .into_iter()
        .filter(|digit| digit.len() == n)
        .map(|digit| BTreeSet::from_iter(digit.chars()))
        .collect()
}

pub fn solve() -> usize {
    load_input(8)
        .into_iter()
        .map(|line| {
            let (observed_digits, output_digits) = line.split_once(" | ").unwrap();
            let mut known_mapping: HashMap<char, char> = HashMap::new();
            let digits: Vec<&str> = observed_digits.split(' ').collect();

            // find a
            let two_dig = get_n_bar_digits(2, &digits).get(0).cloned().unwrap();
            let three_dig = get_n_bar_digits(3, &digits).get(0).cloned().unwrap();
            let a: &char = three_dig.difference(&two_dig).next().unwrap();
            known_mapping.insert(*a, 'a');

            // find g
            let five_digs = get_n_bar_digits(5, &digits);
            let four_dig = get_n_bar_digits(4, &digits).get(0).cloned().unwrap();
            let g = five_digs
                .clone()
                .into_iter()
                .find_map(|dig| {
                    let diff: BTreeSet<char> = dig.difference(&four_dig).cloned().collect();
                    let diff: BTreeSet<char> = diff.difference(&three_dig).cloned().collect();
                    let a = BTreeSet::from([*a]);
                    let diff: BTreeSet<char> = diff.difference(&a).cloned().collect();
                    if diff.len() == 1 {
                        return diff.difference(&three_dig).next().cloned();
                    }
                    None
                })
                .unwrap();
            known_mapping.insert(g, 'g');

            // find d
            let d = five_digs
                .into_iter()
                .find_map(|dig| {
                    let diff: BTreeSet<char> = dig.difference(&three_dig).cloned().collect();
                    let g = BTreeSet::from([g]);
                    let diff: BTreeSet<char> = diff.difference(&g).cloned().collect();
                    if diff.len() == 1 {
                        return diff.into_iter().next();
                    }
                    None
                })
                .unwrap();
            known_mapping.insert(d, 'd');

            // find b
            let six_digs = get_n_bar_digits(6, &digits);
            let b = six_digs
                .clone()
                .into_iter()
                .find_map(|dig| {
                    let diff: BTreeSet<char> = dig.difference(&three_dig).cloned().collect();
                    let dg = BTreeSet::from([d, g]);
                    let diff: BTreeSet<char> = diff.difference(&dg).cloned().collect();
                    if diff.len() == 1 {
                        return diff.into_iter().next();
                    }
                    None
                })
                .unwrap();
            known_mapping.insert(b, 'b');

            // find f
            let seven_dig = get_n_bar_digits(7, &digits).get(0).cloned().unwrap();
            let f = six_digs
                .clone()
                .into_iter()
                .find_map(|dig| {
                    let diff: BTreeSet<char> = seven_dig.difference(&two_dig).cloned().collect();
                    let diff: BTreeSet<char> = dig.difference(&diff).cloned().collect();
                    if diff.len() == 1 {
                        return diff.into_iter().next();
                    }
                    None
                })
                .unwrap();
            known_mapping.insert(f, 'f');

            // find c
            let f_set = BTreeSet::from([f]);
            let c = two_dig.difference(&f_set).next().unwrap();
            known_mapping.insert(*c, 'c');

            // find e
            let seven_dig = get_n_bar_digits(7, &digits).get(0).cloned().unwrap();
            let abcdfg = BTreeSet::from([*a, b, *c, d, f, g]);
            let e = seven_dig.difference(&abcdfg).next().unwrap();
            known_mapping.insert(*e, 'e');

            let output_digits: Vec<&str> = output_digits.split(' ').collect();
            let mapped_digits: Vec<&str> = output_digits
                .into_iter()
                .map(|digit| {
                    let mapped_digit: String = digit
                        .chars()
                        .map(|c| known_mapping.get(&c).unwrap())
                        .sorted()
                        .collect();
                    match mapped_digit.as_str() {
                        "abcefg" => "0",
                        "cf" => "1",
                        "acdeg" => "2",
                        "acdfg" => "3",
                        "bcdf" => "4",
                        "abdfg" => "5",
                        "abdefg" => "6",
                        "acf" => "7",
                        "abcdefg" => "8",
                        "abcdfg" => "9",
                        el => panic!("Invalid element {}", el),
                    }
                })
                .collect();
            let digit = mapped_digits.join("").parse::<usize>().unwrap();
            digit
        })
        .sum::<usize>()
}
