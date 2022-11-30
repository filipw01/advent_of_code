use std::fs::read_to_string;

pub fn find_bus() -> usize {
    let file_content = read_to_string("day13/data.txt").unwrap();
    let lines: Vec<_> = file_content.split("\n").collect();
    let data: Vec<&str> = lines[1].split(',').collect();
    let mut busses: Vec<(usize, usize)> = Vec::new();
    for (delay, bus) in data.iter().enumerate() {
        if *bus == "x" {
            continue;
        }
        let bus_number = bus.parse().unwrap();
        if delay % bus_number == 0 {
            busses.push((bus_number, 0))
        } else {
            busses.push((bus_number, bus_number - (delay % bus_number)))
        }
    }
    busses.sort_by(|a, b| b.cmp(&a));
    let mut index = 1;
    let mut multiplier = busses[0].0;
    let mut prev = busses[0].1;
    while index < busses.len() {
        prev = prev + multiplier;
        if busses[index].1 == prev % busses[index].0 {
            multiplier *= busses[index].0;
            index += 1;
        }
    }
    prev
}
