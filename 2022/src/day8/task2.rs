use crate::day8::task1::Map;

pub fn solution(input: &str) -> usize {
    let map = Map::from(input);
    map.is_tree_visible(1, 2);
    (0..map.get_height())
        .map(|y| {
            (0..map.get_width())
                .map(|x| map.get_scenic_score(x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
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
            8
        );
    }
}
