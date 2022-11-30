use std::collections::HashSet;
use std::fs::read_to_string;

pub fn count_declarations() -> usize {
    let file_content = read_to_string("day6/data.txt").unwrap();
    let declaration_batches: Vec<&str> = file_content.split("\n\n").collect();
    let mut total = 0;
    for declaration_batch in declaration_batches {
        let letters: HashSet<char> = declaration_batch.replace("\n", "").chars().collect();
        total += letters.len();
    }
    total
}
