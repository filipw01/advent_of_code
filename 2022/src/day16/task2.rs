use itertools::Itertools;
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
    let mut paths = vec![(("AA", "AA"), 0, HashSet::new())];
    let mut time_left = 26;

    let top_valves = valves
        .values()
        .sorted_by_key(|valve| valve.flow_rate)
        .rev()
        .enumerate();

    while time_left > 0 {
        time_left -= 1;
        let max_difference: usize = top_valves
            .clone()
            .take(time_left)
            .map(|(index, valve)| valve.flow_rate * (time_left - (index * 5).min(time_left)))
            .sum();
        println!("Max diff: {}", max_difference);
        let best_path = paths
            .clone()
            .into_iter()
            .max_by_key(|(_, time, _)| *time)
            .expect("No paths found");
        println!("best path {:?}", best_path);
        println!("{}", paths.len());
        // println!("{:?}", paths);
        paths = paths
            .into_iter()
            .filter(|(_, flow, _)| *flow as i32 >= best_path.1 as i32 - max_difference as i32)
            .flat_map(|(current_position, pressure_released, visited)| {
                // me
                let my_position = current_position.0;
                let my_current_valve = valves.get(my_position).unwrap();
                let mut my_after_move = my_current_valve
                    .connections
                    .iter()
                    .map(|connection| {
                        (
                            (connection.as_str(), current_position.1),
                            pressure_released,
                            visited.clone(),
                        )
                    })
                    .collect::<Vec<_>>();

                if !visited.contains(my_position) && my_current_valve.flow_rate > 0 {
                    let mut new_visited = visited.clone();
                    new_visited.insert(my_position);
                    let after_release = (
                        current_position,
                        pressure_released + my_current_valve.flow_rate * time_left,
                        new_visited,
                    );
                    my_after_move.append(&mut vec![after_release]);
                }

                // elephant
                let elephant_position = current_position.1;
                let elephant_current_valve = valves.get(elephant_position).unwrap();
                let mut elephant_after_move = my_after_move
                    .clone()
                    .into_iter()
                    .flat_map(|(current_position, pressure_released, visited)| {
                        elephant_current_valve
                            .connections
                            .iter()
                            .map(|connection| {
                                (
                                    (current_position.0, connection.as_str()),
                                    pressure_released,
                                    visited.clone(),
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                if !visited.contains(elephant_position) && elephant_current_valve.flow_rate > 0 {
                    let mut after_release = my_after_move
                        .into_iter()
                        .map(|(current_position, pressure_released, visited)| {
                            let mut new_visited = visited.clone();
                            if new_visited.insert(elephant_position) {
                                (
                                    current_position,
                                    pressure_released
                                        + elephant_current_valve.flow_rate * time_left,
                                    new_visited,
                                )
                            } else {
                                (current_position, pressure_released, new_visited)
                            }
                        })
                        .collect();
                    elephant_after_move.append(&mut after_release);
                }
                // println!("after {:?}", elephant_after_move);
                elephant_after_move
            })
            .collect::<Vec<_>>();
    }
    // println!("best path {:?}", paths);
    let best_path = paths
        .into_iter()
        .max_by_key(|(_, time, _)| *time)
        .expect("No paths found");
    println!("{:?}", best_path.2);
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
            1707
        );
    }
}
