use std::fs::read_to_string;

pub fn find_way() -> usize {
    let file_content = read_to_string("day12/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    let mut x = 10;
    let mut y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;
    for line in lines {
        let instruction = line.chars().nth(0).unwrap();
        let value: i32 = line.get(1..).unwrap().parse().unwrap();
        match instruction {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'L' => match value {
                90 => {
                    let temp = -y;
                    y = x;
                    x = temp;
                }
                180 => {
                    x = -x;
                    y = -y;
                }
                270 => {
                    let temp = y;
                    y = -x;
                    x = temp;
                }
                _ => (),
            },
            'R' => match value {
                90 => {
                    let temp = y;
                    y = -x;
                    x = temp;
                }
                180 => {
                    x = -x;
                    y = -y;
                }
                270 => {
                    let temp = -y;
                    y = x;
                    x = temp;
                }
                _ => (),
            },
            'F' => {
                ship_x += value * x;
                ship_y += value * y;
            }
            _ => unreachable!(),
        }
    }
    (ship_x.abs() + ship_y.abs()) as usize
}
