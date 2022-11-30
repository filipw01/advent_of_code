use std::fs::read_to_string;

pub fn find_way() -> usize {
    let file_content = read_to_string("day12/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut x = 0;
    let mut y = 0;
    let mut rotation = 90;
    for line in lines {
        let instruction = line.chars().nth(0).unwrap();
        let value: i32 = line.get(1..).unwrap().parse().unwrap();
        match instruction {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'L' => rotation -= value,
            'R' => rotation += value,
            'F' => match ((rotation % 360) + 360) % 360 {
                0 => y += value,
                90 => x += value,
                180 => y -= value,
                270 => x -= value,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    (x.abs() + y.abs()) as usize
}
