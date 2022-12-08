#[derive(PartialEq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct Map {
    data: Vec<Vec<usize>>,
}

impl Map {
    pub fn get_width(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.data.len()
    }

    fn get_tree(&self, x: usize, y: usize) -> usize {
        self.data[y][x]
    }

    pub fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let tree = self.get_tree(x, y);
        [
            Direction::Top,
            Direction::Bottom,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(|direction| {
            self.get_trees_in_direction(x, y, direction)
                .iter()
                .any(|&x| x >= tree)
        })
        .any(|x| !x)
    }

    fn get_trees_in_direction(&self, x: usize, y: usize, direction: Direction) -> Vec<usize> {
        let mut trees = vec![];
        let x_start = match direction {
            Direction::Left => 0,
            Direction::Right => x,
            _ => x,
        };
        let x_end = match direction {
            Direction::Left => x,
            Direction::Right => self.get_width() - 1,
            _ => x,
        };
        let y_start = match direction {
            Direction::Top => 0,
            Direction::Bottom => y,
            _ => y,
        };
        let y_end = match direction {
            Direction::Top => y,
            Direction::Bottom => self.get_height() - 1,
            _ => y,
        };
        for x_search in x_start..=x_end {
            for y_search in y_start..=y_end {
                if x_search == x && y_search == y {
                    continue;
                }
                trees.push(self.get_tree(x_search, y_search));
            }
        }
        if direction == Direction::Left || direction == Direction::Top {
            trees.reverse();
        }
        trees
    }

    pub fn get_scenic_score(&self, x: usize, y: usize) -> usize {
        let tree = self.get_tree(x, y);
        [
            Direction::Top,
            Direction::Bottom,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(|direction| {
            let trees = self.get_trees_in_direction(x, y, direction);
            count_visible_trees(&trees, tree)
        })
        .product()
    }
}

fn count_visible_trees(trees: &Vec<usize>, tree_height: usize) -> usize {
    if let Some(index) = trees.iter().position(|&x| x >= tree_height) {
        index + 1
    } else {
        trees.len()
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        Map {
            data: s
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        }
    }
}

pub fn solution(input: &str) -> usize {
    let map = Map::from(input);
    map.is_tree_visible(1, 2);
    (0..map.get_height())
        .map(|y| {
            (0..map.get_width())
                .filter(|&x| map.is_tree_visible(x, y))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "30373
25512
65332
33549
35390
"
            ),
            21
        );
    }
}
