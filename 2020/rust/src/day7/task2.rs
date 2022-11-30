use std::collections::HashMap;
use std::fs::read_to_string;

pub fn count_bags() -> usize {
    let file_content = read_to_string("day7/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split(".\n").collect();
    lines.pop();
    let searched_bag = "shiny gold";
    let mut bags: HashMap<&str, Vec<(usize, String)>> = HashMap::new();
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
                let mut split = bag
                    .trim_matches(|c: char| ['b', 'a', 'g', 's'].contains(&c))
                    .split(' ');
                let count = split.next().unwrap().parse().unwrap();
                let mut name = split.collect::<Vec<_>>();
                name.pop();
                let name = name.join(" ");
                (count, name)
            })
            .collect();
        bags.insert(color, inner_bags);
    }
    return count_inner_bags(searched_bag, &bags, None);
}

fn count_inner_bags(
    searched_bag: &str,
    bags: &HashMap<&str, Vec<(usize, String)>>,
    multiplier: Option<usize>,
) -> usize {
    let mut count = 0;

    for (inner_count, inner_color) in bags.get(searched_bag).unwrap_or(&vec![]) {
        count += (inner_count + count_inner_bags(inner_color.as_str(), bags, Some(*inner_count)))
            * multiplier.unwrap_or(1);
    }
    count
}
