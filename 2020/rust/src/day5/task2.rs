use std::fs::read_to_string;

pub fn find_your_seat() -> isize {
    let file_content = read_to_string("day5/data.txt").unwrap();
    let mut lines: Vec<&str> = file_content.split("\n").collect();
    lines.pop();
    let mut vec = Vec::with_capacity(1023);
    for i in 0..1023 {
        vec.push(i.to_string());
    }
    lines.iter().for_each(|line| {
        let row: String = line
            .get(0..7)
            .unwrap()
            .chars()
            .map(|c| if c == 'F' { '0' } else { '1' })
            .collect();
        let column: String = line
            .get(7..)
            .unwrap()
            .chars()
            .map(|c| if c == 'L' { '0' } else { '1' })
            .collect();
        let row_number = isize::from_str_radix(row.as_str(), 2).unwrap();
        let column_number = isize::from_str_radix(column.as_str(), 2).unwrap();
        let id: usize = (8 * row_number + column_number) as usize;
        vec[id] = ' '.to_string();
    });
    let your_seat = vec
        .join("")
        .trim_matches(char::is_numeric)
        .trim()
        .parse()
        .unwrap();
    your_seat
}
