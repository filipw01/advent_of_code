use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn count_containers() -> usize {
    let file_content = read_to_string("day7/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split(".\n").collect();
    lines.pop();
    let searched_bag = "shiny gold";
    let mut bags: HashMap<&str, Vec<&str>> = HashMap::new();
    lines.pop();
    for line in lines {
        let line: Vec<_> = line.split(" bags contain ").collect();
        let color = line.get(0).unwrap();
        let content = line.get(1).unwrap();
        if *content == "no other bags" {
            continue;
        }
        let inner_bags = content
            .split(", ")
            .into_iter()
            .map(|bag| {
                bag.trim_matches(|c: char| ['b', 'a', 'g', 's'].contains(&c))
                    .trim_matches(char::is_numeric)
                    .trim()
            })
            .collect();
        bags.insert(color, inner_bags);
    }
    return find_containers(searched_bag, bags).len();
}

fn find_containers<'a>(
    searched_bag: &'a str,
    bags: HashMap<&'a str, Vec<&str>>,
) -> HashSet<&'a str> {
    let mut containers = HashSet::new();
    for (color, content) in &bags {
        if content.contains(&searched_bag) {
            containers.insert(*color);
            containers = containers
                .union(&find_containers(color, bags.clone()))
                .map(|x| *x)
                .collect();
        }
    }
    containers
}
