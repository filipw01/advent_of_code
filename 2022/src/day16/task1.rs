use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

struct Valve {
    name: String,
    flow_rate: usize,
    connections: Vec<String>,
}

impl From<&str> for Valve {
    fn from(s: &str) -> Self {
        let s = s.split_once("Valve ").unwrap().1;
        let (name, s) = s.split_once(" has flow rate=").unwrap();
        let (flow_rate, s) = s
            .split_once("; tunnels lead to valves ")
            .unwrap_or_else(|| s.split_once("; tunnel leads to valve").unwrap());
        let connections = s.split(", ").map(|s| s.trim().to_string()).collect();
        Self {
            name: name.to_string(),
            flow_rate: flow_rate.parse().unwrap(),
            connections,
        }
    }
}

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valve {} flow={}; tunnels to {}",
            self.name,
            self.flow_rate,
            self.connections.join(", ")
        )
    }
}

pub fn solution(input: &str) -> usize {
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(Valve::from)
        .map(|v| (v.name.clone(), v))
        .collect();
    let mut paths = vec![("AA", 0, HashSet::from(["AA"]))];
    let mut time_left = 30;

    let max_valve = valves
        .values()
        .max_by_key(|v| v.flow_rate)
        .expect("No valves found")
        .flow_rate;
    while time_left > 0 {
        time_left -= 1;

        let max_difference = (time_left + 1) * max_valve;
        let best_path = paths
            .clone()
            .into_iter()
            .max_by_key(|(_, time, _)| *time)
            .expect("No paths found");
        println!("best path {:?}", best_path);
        paths = paths
            .into_iter()
            .filter(|(_, flow, _)| *flow as i32 >= best_path.1 as i32 - max_difference as i32)
            .flat_map(|(current_position, pressure_released, visited)| {
                let current_valve = valves.get(current_position).unwrap();
                let mut after_move = current_valve
                    .connections
                    .iter()
                    .map(|connection| (connection.as_str(), pressure_released, visited.clone()))
                    .collect::<Vec<_>>();

                if !visited.contains(current_position) {
                    let mut new_visited = visited.clone();
                    new_visited.insert(current_position);
                    let after_release = (
                        current_position,
                        pressure_released + current_valve.flow_rate * time_left,
                        new_visited,
                    );
                    after_move.append(&mut vec![after_release]);
                }
                after_move
            })
            .collect::<Vec<_>>();
    }
    let best_path = paths
        .into_iter()
        .max_by_key(|(_, time, _)| *time)
        .expect("No paths found");
    best_path.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"
            ),
            1651
        );
    }
}
