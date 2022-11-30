use std::fs::read_to_string;

pub fn break_code() -> usize {
    let file_content = read_to_string("day9/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let lines: Vec<usize> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let weakness = code_weakness(&lines);
    let mut start = 0;
    let mut end = 1;
    loop {
        let continuous_sum = lines[start..end].iter().fold(0, |acc, line| acc + line);
        match continuous_sum {
            s if s > weakness => start += 1,
            s if s < weakness => end += 1,
            s if s == weakness => {
                return lines[start..end].iter().min().unwrap()
                    + lines[start..end].iter().max().unwrap()
            }
            _ => unreachable!(),
        }
    }
}

fn code_weakness(lines: &Vec<usize>) -> usize {
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
