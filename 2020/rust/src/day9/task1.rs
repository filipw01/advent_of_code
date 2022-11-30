use std::fs::read_to_string;

pub fn code_weakness() -> usize {
    let file_content = read_to_string("day9/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let lines: Vec<usize> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let mut i = 0;
    loop {
        if !has_sum(&lines[i..i + 25], lines.get(i + 25).unwrap()) {
            return *lines.get(i + 25).unwrap();
        }
        i += 1;
    }
}

fn has_sum(numbers: &[usize], solution: &usize) -> bool {
    numbers.iter().enumerate().any(|(index_1, item_1)| {
        numbers.iter().enumerate().any(|(index_2, item_2)| {
            if index_1 == index_2 {
                return false;
            }
            if item_1 + item_2 == *solution {
                return true;
            }
            false
        })
    })
}
