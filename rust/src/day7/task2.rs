use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(7);
    let crabs: Vec<usize> = lines[0]
        .split(',')
        .into_iter()
        .map(|crab| crab.parse().unwrap())
        .collect();
    let min_crab = crabs.iter().min().unwrap();
    let max_crab = crabs.iter().max().unwrap();
    let mut min_fuel: usize = usize::MAX;
    for test_pos in *min_crab..(*max_crab + 1) {
        let fuel = crabs
            .iter()
            .map(|pos| distance_to_fuel_cost((*pos as isize - test_pos as isize).abs() as usize))
            .sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

fn distance_to_fuel_cost(distance: usize) -> usize {
    let mut distance = distance;
    let mut fuel = 0;
    while distance > 0 {
        fuel += distance;
        distance -= 1;
    }
    fuel
}
