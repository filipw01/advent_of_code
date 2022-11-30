use std::collections::HashSet;
use std::fs::read_to_string;

pub fn count_declarations() -> usize {
    let file_content = read_to_string("day6/data.txt").unwrap();
    let declaration_batches: Vec<&str> = file_content.split("\n\n").collect();
    let mut total = 0;
    for declaration_batch in declaration_batches {
        let mut declared: HashSet<char> = declaration_batch
            .split('\n')
            .next()
            .unwrap()
            .chars()
            .collect();
        declaration_batch
            .split('\n')
            .skip(1)
            .for_each(|declaration| {
                if declaration == "" {
                    return;
                }
                let temp_set: HashSet<char> = declaration.chars().collect();
                declared = &declared & &temp_set
            });
        total += declared.len();
    }
    total
}
