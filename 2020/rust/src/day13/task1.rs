use std::fs::read_to_string;

pub fn find_bus() -> usize {
    let file_content = read_to_string("day13/data.txt").unwrap();
    let lines: Vec<_> = file_content.split("\n").collect();
    let (timestamp, busses) = (lines[0], lines[1]);
    let busses: Vec<usize> = busses
        .split(',')
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse().unwrap())
        .collect();
    let mut minimum_id = busses[0];
    let mut minimum_delay = busses[0];
    for bus in busses {
        let delay = bus - timestamp.parse::<usize>().unwrap() % bus;
        if delay < minimum_delay {
            minimum_delay = delay;
            minimum_id = bus;
        }
    }
    minimum_delay * minimum_id
}
