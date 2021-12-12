use crate::utils::load_input;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Cave {
    is_big: bool,
    linking_to: Vec<String>,
}

pub fn solve() -> usize {
    let lines = load_input(12);
    let mut caves: HashMap<String, Cave> = HashMap::new();
    lines.into_iter().for_each(|line| {
        let (from, to) = line.split_once('-').unwrap();
        add_cave(&mut caves, from, to);
        add_cave(&mut caves, to, from);
    });
    let visited = HashSet::new();
    traverse_caves(&caves, "start", &visited, false)
}

fn traverse_caves(
    caves: &HashMap<String, Cave>,
    current: &str,
    visited: &HashSet<String>,
    visited_twice: bool,
) -> usize {
    let cave = caves.get(current).unwrap();
    if current == "end" {
        return 1;
    }
    cave.linking_to
        .clone()
        .into_iter()
        .filter(|linked_cave| {
            (!visited_twice || !visited.contains(linked_cave)) && linked_cave != "start"
        })
        .map(|linked_cave| {
            let mut new_visited = visited.clone();
            if !cave.is_big {
                new_visited.insert(current.to_string());
            }
            traverse_caves(
                caves,
                linked_cave.as_str(),
                &new_visited,
                visited_twice || visited.contains(linked_cave.as_str()),
            )
        })
        .sum()
}

fn add_cave(caves: &mut HashMap<String, Cave>, cave_from: &str, cave_to: &str) {
    let cave = caves.get_mut(cave_from);
    if let Some(cave) = cave {
        cave.linking_to.push(cave_to.to_string());
    } else {
        caves.insert(
            cave_from.to_string(),
            Cave {
                is_big: cave_from.chars().next().unwrap().is_ascii_uppercase(),
                linking_to: vec![cave_to.to_string()],
            },
        );
    }
}
