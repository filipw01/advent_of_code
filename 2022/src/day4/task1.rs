use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let lines = input.lines().map_into::<String>();
    lines
        .filter(|line| {
            let ((start_1, end_1), (start_2, end_2)) = parse_line(line);
            (start_1 <= start_2 && end_1 >= end_2) || (start_2 <= start_1 && end_2 >= end_1)
        })
        .count()
}

pub fn parse_line(line: &str) -> ((usize, usize), (usize, usize)) {
    line.split(',')
        .map(|s| {
            s.split('-')
                .map(|i| i.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
            ),
            2
        );
    }
}
