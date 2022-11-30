use std::collections::HashMap;
use std::fs::read_to_string;

pub fn find_valid_combinations() -> usize {
    let file_content = read_to_string("day10/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut adapters = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();
    adapters.sort();
    let max_adapters = adapters.last().unwrap();
    let mut results: HashMap<usize, usize> = HashMap::new();
    results.insert(0, 1);
    adapters.iter().for_each(|adapter| {
        let mut results_clone = results.clone();
        results.iter().for_each(|(result, _)| {
            if result + 1 == *adapter || result + 2 == *adapter || result + 3 == *adapter {
                let new_result = results_clone.get(adapter).unwrap_or(&0)
                    + results_clone.get(result).unwrap_or(&0);
                results_clone.insert(*adapter, new_result);
            }
        });
        results = results_clone;
    });
    *results.get(max_adapters).unwrap()
}
